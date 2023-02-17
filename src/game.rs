use std::path::Path;
use std::path::PathBuf;

type Pck = PathBuf;

use iced::widget::container;
use iced::widget::Container;
use iced::Length;
use iced::widget::image;
use iced::widget::text;

use crate::os::Message;

use iced::alignment;
use iced::event::Event;
use iced::executor;
use iced::keyboard::KeyCode;
use iced::subscription;
use iced::widget::{button, checkbox, Column};
use iced::window;
use iced::{Alignment, Application, Command, Element, Settings, Subscription, Theme};
use iced::Sandbox;
use iced::widget::{row, Row, column};

pub const GAME_EXT: &str = "pck";

#[derive(Debug, PartialEq)]
pub struct Game {
    pck: Pck,
}

impl Game {
    pub fn new(pck: Pck) -> Self {
        Self { 
            pck: pck,
        }
    }

    /// References the game's .pck file path.
    pub fn get_pck(&self) -> &Pck {
        &self.pck
    }

    /// Checks if the `path` is a valid Godot game file.
    pub fn is_game_file<T: AsRef<Path> + ?Sized>(path: &T) -> bool {
        std::path::Path::is_file(&path.as_ref())
            && match path.as_ref().extension() {
                Some(ext) => ext == GAME_EXT,
                None => false,
            }
    }

    pub fn get_icon_path(&self) -> Option<PathBuf> {
        //&self.pck.file_stem()
        let mut icon_path = self.pck.clone();
        icon_path.set_extension("png");
        if icon_path.exists() == true && icon_path.is_file() == true {
            Some(icon_path)
        } else {
            None
        }
    }

    pub fn get_name(&self) -> &str {
        // @todo: provide a `from` function with Result<T,E> to ensure a loaded game has a name
        &self.pck.file_stem().as_ref().unwrap().to_str().unwrap()
    }
}

impl<'a> Game {
    pub fn container(title: Option<&str>) -> Column<'a, Message> {
        match title {
            Some(s) => column![text(s).size(50)].spacing(20),
            None => column![].spacing(20)
        }
    }

    pub fn blank() -> Column<'a, Message> {
        Self::container(None)
            .push(
                container(image("images/empty.png")
                .width(Length::Units(256))
                .height(Length::Units(256)))
                .center_x()         
            )
            .push(
                text("")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .align_items(Alignment::Center)
    }

    pub fn draw(&self, selected: bool) -> Column<'a, Message> {
        Self::container(None)
            .push(
                container(
                    image(self.get_icon_path().unwrap())
                    .width(Length::Units(256 + if selected == true { 128 } else { 0 }))
                    .height(Length::Units(256 + if selected == true { 128 } else { 0 }))
                ).center_x()
            )
            .push(
                text(format!("{}", self.get_name()))
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .align_items(Alignment::Center)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_is_game_file_good() {
        assert_eq!(Game::is_game_file("testenv/GAMESTICK/platformer.pck"), true);
        assert_eq!(Game::is_game_file("testenv/GAMESTICK/fsm.pck"), true);
    }

    #[test]
    fn ut_is_game_file_bad() {
        // file is not .pck
        assert_eq!(Game::is_game_file("Cargo.toml"), false);
        // file does not exist
        assert_eq!(Game::is_game_file("testenv/GAMESTICK/missing.pck"), false);
        assert_eq!(Game::is_game_file("./testenv/GAMESTICK/game"), false);
    }
}
