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
use iced::event::Event;
use iced::executor;
use iced::keyboard::KeyCode;
use iced::subscription;
use iced::widget::{button, checkbox, container, text, Column};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use iced::Sandbox;
use iced::widget::{row, Row, column};

impl Application for Os {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Os, Command<Message>) {
        (
            Self {
                engine: Engine::new(),
                library: GameStick::load(&PathBuf::from("./testenv/GAMESTICK")),
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
        // We use a column: a simple vertical layout
        column![        
            row![
                // show the game's name
                text(self.library.get(0).unwrap().get_name()).size(if self.count == 0 { 50 } else { 25 }),
                text(self.library.get(1).unwrap().get_name()).size(if self.count == 1 { 50 } else { 25 }),
            ]
            .padding(50)
            .spacing(50)
            .align_items(Alignment::Center),
            button("play").on_press(Message::PlayGame),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}