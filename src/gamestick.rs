use crate::game::Game;
use crate::game;
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

    /// Read the [GameStick]'s filesystem for Godot game files.
    /// 
    /// Assumes the [GameStick] is present and exists from the console's perspective.
    pub fn load(root: &PathBuf) -> Vec<Game> {

        let glob_pattern = glob::Pattern::new(&format!("{}/**/*.{}", root.to_str().unwrap(), game::GAME_EXT))
            .expect("Failed to read glob pattern");
        
        glob::glob(&glob_pattern.as_str()).unwrap().filter_map(|entry| {
            match entry {
                Ok(path) => {
                    Some(Game::new(path))
                },
                Err(e) => { 
                    eprintln!("{:?}", e); 
                    None 
                },
            }
        }).collect()
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
            let mut root = PathBuf::from("/media/");
            match dirs::home_dir() {
                Some(hp) => {
                    root.push(hp.file_name().unwrap());
                    root.push("/GAMESTICK")
                }
                None => root.push("GAMESTICK"),
            }
            root
        } else if cfg!(target_os = "macos") == true {
            PathBuf::from("/Volumes/GAMESTICK")
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

            match command {
                // successfully ejected
                Ok(_) => true,
                // failed to eject
                Err(_) => false,
            }
        // not available to eject
        } else {
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
    fn ut_exists() {
        let gs = GameStick::test_new();
        assert_eq!(gs.exists(), true);
    }

    #[test]
    fn ut_load_library() {
        let library = GameStick::load(GameStick::test_new().get_path());

        assert_eq!(library.len(), 2);
        assert_eq!(library.iter().find(|f| f.get_name() == "fsm" ).is_some(), true);
        assert_eq!(library.iter().find(|f| f.get_name() == "gd-paint" ).is_some(), true);
    }
}
