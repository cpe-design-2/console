mod engine;
mod env;
mod game;
mod gamestick;

// Reference: https://github.com/iced-rs/iced/tree/0.7/examples/events
use iced::alignment;
use iced::event::Event;
use iced::executor;
use iced::subscription;
use iced::widget::{button, checkbox, container, text, Column};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};

use engine::Engine;
use game::Game;
use std::path::PathBuf;

pub fn go() -> u8 {
    println!("Booting up goco ...");
    match Events::run(Settings {
        exit_on_close_request: false,
        ..Settings::default()
    }) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            101
        }
    }
}

#[derive(Debug, Default)]
struct Events {
    last: Vec<Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Toggled(bool),
    Play,
    Exit,
}

impl Application for Events {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        (
            Events::default(),
            // determine at run-time the full-screen mode
            match std::env::var(env::GOCO_NO_FULLSCREEN) {
                Ok(_) => Command::none(),
                Err(_) => iced::window::set_mode::<Message>(window::Mode::Fullscreen),
            },
        )
    }

    fn title(&self) -> String {
        String::from("Events - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }

                Command::none()
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::close()
                } else {
                    Command::none()
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;

                Command::none()
            }
            Message::Exit => window::close(),
            Message::Play => {
                println!("play game!");
                let gd = Engine::new();
                gd.play_game(&Game::new(PathBuf::from("./testenv/GAMESTICK/fsm2.pck")));
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let events = Column::with_children(
            self.last
                .iter()
                .map(|event| text(format!("{:?}", event)).size(40))
                .map(Element::from)
                .collect(),
        );

        let toggle = checkbox("Listen to runtime events", self.enabled, Message::Toggled);

        let exit = button(
            text("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(Length::Units(100))
        .padding(10)
        .on_press(Message::Exit);

        let play = button(
            text("Play")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(Length::Units(100))
        .padding(10)
        .on_press(Message::Play);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit)
            .push(play);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
