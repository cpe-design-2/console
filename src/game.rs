use std::env;
use std::path::Path;
use std::path::PathBuf;

use iced::Alignment;
use iced::widget::{Column, image, text, container};

use crate::env::GOCO_ROOT;
use crate::os::Message;


type Pck = PathBuf;

/// The default size for icons to be displayed in the game library.
const ICON_SIZE: u16 = 256;

/// The supported file extension for Godot games.
pub const GAME_EXT: &str = "pck";

/// The supported file extension for image loading.
pub const ICON_EXT: &str = "png";

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

    /// Attempts to extract the game's icon file path.
    /// 
    /// If the result is some [PathBuf], then it is safe to assume the path
    /// exists and is a file.
    pub fn get_icon_path(&self) -> Option<PathBuf> {
        let mut icon_path = self.pck.clone();
        // replace the extension with the icon extension
        icon_path.set_extension(ICON_EXT);
        // verify the path exists and is a file
        if icon_path.exists() == true && icon_path.is_file() == true {
            Some(icon_path)
        } else {
            None
        }
    }

    /// Returns the console's included empty icon to display when no icon is present.
    /// 
    /// Reads from the `GOCO_ROOT` environment variable to determine the base directory
    /// for finding the `assets/empty.png` file.
    fn empty_icon() -> PathBuf {
        PathBuf::from(format!("{}/assets/empty.png", env::var_os(GOCO_ROOT).unwrap_or(".".into()).to_string_lossy()))
    }

    pub fn get_name(&self) -> &str {
        // @todo: provide a `from` function with Result<T,E> to ensure a loaded game has a name
        &self.pck.file_stem().as_ref().unwrap().to_str().unwrap()
    }
}

impl<'a> Game {
    pub fn container(title: Option<&str>) -> Column<'a, Message> {
        match title {
            Some(s) => iced::widget::column![text(s).size(50)].spacing(20),
            None => iced::widget::column![].spacing(20)
        }
    }

    /// Assembles the container to display an empty slot for a [Game] in the console's main library screen.
    /// The function returns a blank icon and no text, but in the same format as a valid game would be.
    pub fn blank() -> Column<'a, Message> {
        Self::container(None)
            .push(
                container(image(Self::empty_icon())
                .width(ICON_SIZE)
                .height(ICON_SIZE))
                .center_x()         
            )
            .push(
                text("")
                .vertical_alignment(iced::alignment::Vertical::Center)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .align_items(Alignment::Center)
    }

    /// Assembles the container to display the [Game] in the console's main library screen.
    /// If `selected`, then the game's icon will be enlarged.
    pub fn draw(&self, selected: bool) -> Column<'a, Message> {
        Self::container(None)
            .push(
                container(
                    image(self.get_icon_path().unwrap_or(Self::empty_icon()))
                    .width((ICON_SIZE as f32 * if selected == true { 1.5 } else { 1.0 }) as u16)
                    .height((ICON_SIZE as f32 * if selected == true { 1.5 } else { 1.0 }) as u16)
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
        assert_eq!(Game::is_game_file("testenv/GAMESTICK/Super Platformer.pck"), true);
        assert_eq!(Game::is_game_file("testenv/GAMESTICK/Finite State Machine.pck"), true);
    }

    #[test]
    fn ut_is_game_file_bad() {
        // file is not .pck
        assert_eq!(Game::is_game_file("Cargo.toml"), false);
        // file does not exist
        assert_eq!(Game::is_game_file("testenv/GAMESTICK/missing.pck"), false);
        assert_eq!(Game::is_game_file("./testenv/GAMESTICK/game"), false);
    }

    #[test]
    fn ut_get_icon_path_some() {
        let vg = Game::new("testenv/GAMESTICK/Finite State Machine.pck".into());
        assert_eq!(vg.get_icon_path(), Some("testenv/GAMESTICK/Finite State Machine.png".into()));
    }

    #[test]
    fn ut_get_icon_path_none() {
        let vg = Game::new("testenv/GAMESTICK/game.pck".into());
        assert_eq!(vg.get_icon_path(), None);
    }
}
