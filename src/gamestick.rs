use crate::game;
use crate::game::Game;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct GameStick {
    path: PathBuf,
    library: Vec<Game>,
}

impl GameStick {
    /// Creates a new [GameStick] structure.
    pub fn new() -> Self {
        Self {
            path: Self::determine_gamestick_path(),
            library: Vec::new(),
        }
    }

    /// Check if the operating system has permissions to read the root directory.
    pub fn can_read_dir(&self) -> bool {
        std::path::Path::read_dir(&self.path).is_ok()
    }

    /// Read the [GameStick]'s filesystem for Godot game files.
    ///
    /// Assumes the [GameStick] is present and exists from the console's perspective.
    pub fn load(root: &PathBuf) -> Vec<Game> {
        let glob_pattern = glob::Pattern::new(&format!(
            "{}/**/*.{}",
            root.to_str().unwrap(),
            game::GAME_EXT
        ))
        .expect("Failed to read glob pattern");

        // collect all games on the drive
        glob::glob(&glob_pattern.as_str())
            .unwrap()
            .filter_map(|entry| match entry {
                Ok(path) => match Game::is_game_file(&path) {
                    true => Some(Game::new(path)),
                    false => None,
                }
                Err(e) => {
                    eprintln!("error: {:?}", e);
                    None
                }
            })
            .collect()
    }

    /// References the root path where to search for the [GameStick].
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Creates the base path where a bootable set of games can be found when a USB
    /// flash drive is plugged into the computer.
    ///
    /// Supports `linux` os and `macos` os paths.
    fn determine_gamestick_path() -> PathBuf {
        if cfg!(target_os = "linux") == true {
            let mut root = PathBuf::from("/media");
            match dirs::home_dir() {
                Some(hp) => root.push(hp.file_name().unwrap()),
                None => ()
            }
            root.push("GAMESTICK");
            root
        } else if cfg!(target_os = "macos") == true {
            PathBuf::from("/Volumes/GAMESTICK")
        } else if cfg!(target_os = "windows") == true {
            PathBuf::from("D:/")
        } else {
            panic!("unsupported operating system")
        }
    }

    /// Checks if the [GameStick] is available on the current filesystem.
    pub fn exists(&self) -> bool {
        std::path::Path::exists(&self.path)
    }

    /// Attempts to eject the [GameStick] if the drive is available on the current filesystem.
    ///
    /// Returns `true` upon successfull operation and `false` upon failed operation.
    pub fn eject(&self) -> bool {
        if self.exists() == true {
            #[cfg(target_os = "macos")]
            let command = std::process::Command::new("diskutil")
                .arg("unmount")
                .arg(&self.path)
                .spawn();
            #[cfg(target_os = "linux")]
            let command = std::process::Command::new("umount").arg(&self.path).spawn();
            #[cfg(target_os = "windows")]
            let command = std::process::Command::new("").spawn();
            
            match command {
                // successfully ejected
                Ok(_) => true,
                // failed to eject
                Err(e) => {
                    eprintln!("error: {}", e);
                    false
                },
            }
        // not available to eject
        } else {
            eprintln!("error: The drive is not available to eject");
            false
        }
    }
}

#[cfg(test)]
impl GameStick {
    /// Creates a new [GameStick] at a directory on the current filesystem for testing
    /// purposes.
    pub fn test_new() -> Self {
        Self {
            path: PathBuf::from("testenv/GAMESTICK"),
            library: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_combining_paths() {
        let mut root = PathBuf::from("/media");
        root.push(PathBuf::from("home/rpi3/").file_name().unwrap());
        root.push("GAMESTICK");
        assert_eq!(root, PathBuf::from("/media/rpi3/GAMESTICK"));
    }

    #[test]
    fn ut_exists() {
        let gs = GameStick::test_new();
        assert_eq!(gs.exists(), true);
    }

    #[test]
    fn ut_load_library() {
        let library = GameStick::load(GameStick::test_new().get_path());

        assert_eq!(library.len(), 4);
        assert_eq!(
            library.iter().find(|f| f.get_name() == "Finite State Machine").is_some(),
            true
        );
        assert_eq!(
            library.iter().find(|f| f.get_name() == "Pong").is_some(),
            true
        );
        assert_eq!(
            library.iter().find(|f| f.get_name() == "Dodge The Creeps").is_some(),
            true
        );
        assert_eq!(
            library
                .iter()
                .find(|f| f.get_name() == "Super Platformer")
                .is_some(),
            true
        );
    }
}
