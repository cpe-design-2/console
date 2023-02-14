use std::path::Path;
use std::path::PathBuf;

type Pck = PathBuf;

pub const GAME_EXT: &str = "pck";

#[derive(Debug, PartialEq)]
pub struct Game {
    pck: Pck,
}

impl Game {
    pub fn new(pck: Pck) -> Self {
        Self { pck: pck }
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

    pub fn get_name(&self) -> &str {
        // @todo: provide a `from` function with Result<T,E> to ensure a loaded game has a name
        &self.pck.file_stem().as_ref().unwrap().to_str().unwrap()
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
