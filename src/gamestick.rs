use std::path::PathBuf;
use crate::game::Game;

#[derive(Debug, PartialEq)]
pub struct GameStick {
    path: PathBuf,
    library: Vec<Game>,
}

impl GameStick {
    /// Creates a new [GameStick] structure.
    pub fn new() -> Self {
        Self {
            path: Self::get_gamestick_path(),
            library: Vec::new(),
        }
    }

    #[cfg(test)]
    /// Creates a new [GameStick] at a directory on the current filesystem for testing
    /// purposes.
    pub fn test_new() -> Self {
        Self {
            path: PathBuf::from("testenv/GAMESTICK"),
            library: Vec::new(),
        }
    }

    /// Creates the base path where a bootable set of games can be found when a USB
    /// flash drive is plugged into the computer.
    /// 
    /// Supports `linux` os and `macos` os paths.
    fn get_gamestick_path() -> PathBuf {
        if cfg!(target_os="linux") == true {
            let mut root = PathBuf::from("/media/");
            match dirs::home_dir() {
                Some(hp) => {
                    root.push(hp.file_name().unwrap()); root.push("/GAMESTICK")
                }
                None => root.push("GAMESTICK"),
            }
            root
        } else if cfg!(target_os="macos") == true {
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
            #[cfg(target_os="macos")]
            let command = std::process::Command::new("diskutil").arg("unmount").arg(&self.path).spawn();
            #[cfg(target_os="linux")]
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
mod tests {
    use super::*;

    #[test]
    fn ut_exists() {
        let gs = GameStick::test_new();
        assert_eq!(gs.exists(), true);
    }
}