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

// define the possible user interactions of the main screen operating system
#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    PlayGame,
    SelectNextGame,
    SelectPrevGame,
}

use std::path::PathBuf;

use iced::alignment;
use iced::widget::Container;
use iced::event::Event;
use iced::executor;
use iced::keyboard::KeyCode;
use iced::subscription;
use iced::widget::{button, checkbox, container, text, Column};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use iced::widget::{row, Row, column};


impl Os {
    /// Access the games surrounding the current index.
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
}

impl Application for Os {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Os, Command<Message>) {
        (
            Self {
                engine: Engine::new(),
                library: GameStick::load(&PathBuf::from(format!("{}/testenv/GAMESTICK", env!("CARGO_MANIFEST_DIR")))),
                count: 0,
            },
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
            Message::SelectNextGame => {
                // cap at len()-1
                if self.count + 1 < self.library.len() {
                    self.count += 1;
                }
                Command::none()
            }
            Message::SelectPrevGame => {
                // cap at 0
                if self.count >= 1 {
                    self.count -= 1;
                }
                Command::none()
            }
            Message::PlayGame => {
                // guaranteed to have `count` as a valid index for game library
                self.engine.play_game(self.library.get(self.count).unwrap());
                Command::none()
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::close()
                } else {
                    // println!("{:?}", event);
                    // handle keyboard input
                    if let Event::Keyboard(iced::keyboard::Event::KeyPressed { key_code, modifiers }) = event {
                        match key_code {
                            // right
                            KeyCode::D => {
                                println!("move right");
                                // cap at len()-1
                                if self.count + 1 < self.library.len() {
                                    self.count += 1;
                                }
                            },
                            // left
                            KeyCode::A => {
                                println!("move left");
                                // cap at 0
                                if self.count >= 1 {
                                    self.count -= 1;
                                }
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
            }
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
                match nearby_games[0] { Some(g) => { Container::new(g.draw(false)) } None => { Container::new(Game::blank()) } },
                // the middle index (`1`) is the selected game
                match nearby_games[1] { Some(g) => { Container::new(g.draw(true)) } None => { Container::new(Game::blank()) } },
                match nearby_games[2] { Some(g) => { Container::new(g.draw(false)) } None => { Container::new(Game::blank()) } },
            ]
            .spacing(64),
            button("play").on_press(Message::PlayGame),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(64)
        .align_items(Alignment::Center)
        .into()
    }
}