use std::path::PathBuf;

use iced::event::Event;
use iced::executor;
use iced::keyboard::KeyCode;
use iced::subscription;
use iced::window;
use iced::keyboard::Event::KeyPressed;
use iced::{Alignment, Application, Command, Element, Length, Subscription, Theme};
use iced::widget::{row, column, button, Container};

use crate::engine::Engine;
use crate::game::Game;
use crate::gamestick::GameStick;

// model the state of the application
#[derive(Debug, PartialEq)]
pub struct Os {
    // the backend Godot game engine to invoke for playing games
    engine: Engine,
    /// List of all available loaded games
    library: Vec<Game>,
    /// Determine the current selected game in the list
    count: usize,
}

impl Os {
    /// Constructs a new [Os] structure.
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            library: GameStick::load(&PathBuf::from(format!("{}/testenv/GAMESTICK", env!("CARGO_MANIFEST_DIR")))),
            count: 0,
        }
    }

    /// Access the games surrounding the current index.
    /// 
    /// Returns an element with entry `None` if the index is out of bounds of the
    /// currently loaded game library.
    fn get_nearby_games(&self) -> [Option<&Game>; 3] {
        let mut result = [None, None, None];
        // check at one level before and one level after the current index
        result[0] = if self.count == 0 {
            None
        } else {
            self.library.get(self.count - 1)
        };

        result[1] = self.library.get(self.count);

        result[2] = self.library.get(self.count + 1);

        result
    }

    fn shift_shelf_right(&mut self) -> bool {
        // cap at len()-1
        let able_to_shift = self.count + 1 < self.library.len();
        if able_to_shift == true {
            self.count += 1;
        }
        able_to_shift
    }

    fn shift_shelf_left(&mut self) -> bool {
        let able_to_shift = self.count >= 1;
        // cap at 0
        if able_to_shift == true {
            self.count -= 1;
        }
        able_to_shift
    }
}

// define the possible user interactions of the main screen operating system
#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    PlayGame,
}

impl Application for Os {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Os, Command<Message>) {
        (
            Os::new(),
            // determine at run-time the full-screen mode
            match std::env::var(crate::env::GOCO_NO_FULLSCREEN) {
                Ok(_) => Command::none(),
                Err(_) => iced::window::set_mode::<Message>(window::Mode::Fullscreen),
            },
        )
    }

    fn title(&self) -> String {
        String::from("GOCO")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlayGame => {
                // guaranteed to have `count` as a valid index for game library
                self.engine.play_game(self.library.get(self.count).unwrap());
                Command::none()
            }
            Message::EventOccurred(Event::Window(event)) => {
                if window::Event::CloseRequested == event {
                    window::close()
                } else {
                    Command::none()
                }
            }
            // handle keyboard input
            Message::EventOccurred(Event::Keyboard(event)) => {
                if let KeyPressed { key_code, modifiers: _ } = event {
                    match key_code {
                        // right
                        KeyCode::D => {
                            self.shift_shelf_right();
                        },
                        // left
                        KeyCode::A => {
                            self.shift_shelf_left();
                        },
                        // down
                        KeyCode::S => {

                        },
                        // up
                        KeyCode::W => {

                        },
                        // action key (spacebar)
                        KeyCode::Space => {
                            // guaranteed to have `count` as a valid index for game library
                            self.engine.play_game(self.library.get(self.count).unwrap());
                        }
                        _ => (),
                    }
                }
                Command::none()
            }
            _ => Command::none()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let nearby_games = self.get_nearby_games();
        // use a column: a simple vertical layout
        column![  
            // display the game's in a row      
            row![
                // game appear on the LHS
                match nearby_games[0] { Some(g) => { Container::new(g.draw(false)) } None => { Container::new(Game::blank()) } },
                // the middle index (`1`) is the selected game
                match nearby_games[1] { Some(g) => { Container::new(g.draw(true)) } None => { Container::new(Game::blank()) } },
                // game appear on the RHS
                match nearby_games[2] { Some(g) => { Container::new(g.draw(false)) } None => { Container::new(Game::blank()) } },
            ]
            .spacing(64),
            button("PLAY").on_press(Message::PlayGame),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(64)
        .align_items(Alignment::Center)
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_load_nearby_games() {
        let mut os = Os {
            engine: Engine::new(),
            library: GameStick::load(&PathBuf::from(format!("{}/testenv/GAMESTICK", env!("CARGO_MANIFEST_DIR")))),
            count: 0,
        };
        {
            let shelf = os.get_nearby_games();
            assert_eq!(shelf[0].is_none(), true);
            assert_eq!(shelf[1].unwrap(), &os.library[0]);
            assert_eq!(shelf.iter().filter(|p| p.is_some()).count(), 2);
        }
        os.shift_shelf_right();
        {
            let shelf = os.get_nearby_games();
            assert_eq!(shelf[0].unwrap(), &os.library[0]);
            assert_eq!(shelf[1].unwrap(), &os.library[1]);
            assert_eq!(shelf.iter().filter(|p| p.is_some()).count(), 3);
        }
        os.shift_shelf_left();
        {
            let shelf = os.get_nearby_games();
            assert_eq!(shelf[0].is_none(), true);
            assert_eq!(shelf[1].unwrap(), &os.library[0]);
            assert_eq!(shelf.iter().filter(|p| p.is_some()).count(), 2);
        }
        os.shift_shelf_left();
        {
            let shelf = os.get_nearby_games();
            assert_eq!(shelf[0].is_none(), true);
            assert_eq!(shelf[1].unwrap(), &os.library[0]);
            assert_eq!(shelf.iter().filter(|p| p.is_some()).count(), 2);
        }
    }
}