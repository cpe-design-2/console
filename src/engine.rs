use std::path::Path;

use crate::env;
use crate::game::Game;

#[derive(Debug, PartialEq)]
pub struct Engine {
    /// full filepath to the Godot game engine executable
    exe: String,
    /// the child process ID for the current game being ran on the engine
    child: Option<u32>,
}

impl Engine {
    /// Creates a new backend [Engine] structure.
    ///
    /// The path to the engine exe can be defined with the `GOCO_GODOT_PATH` environment
    /// variable. If the environment variable is not set, it assumes the executable can
    /// be invoked as "godot".
    pub fn new() -> Self {
        Self {
            exe: match std::env::var(env::GOCO_GODOT_PATH) {
                Ok(val) => val,
                Err(_) => String::from("godot"),
            },
            child: None
        }
    }

    /// Invokes the Godot game engine and starts a [Game] `game`.
    /// 
    /// This function internally checks if a game is already running and will only
    /// invoke a game if no child process is found.
    pub fn play_game(&mut self, game: &Game) -> () {
        if self.is_in_game() == false {
            // check if the executable exists
            match Path::new(&self.exe).is_file() {
                false => {
                    eprintln!("error: Godot executable path {:?} does not exist", self.exe);
                }
                true => {
                    match std::process::Command::new(&self.exe)
                        .arg("--fullscreen")
                        .arg("--always-on-top")
                        .arg("--main-pack")
                        .arg(game.get_pck())
                        .spawn()
                    {
                        // store the child's ID for future usage
                        Ok(child) => {
                            self.child = Some(child.id());
                        }
                        Err(e) => {
                            eprintln!("error: {}", e);
                        },
                    }
                }
            }
        } else {
            eprintln!("error: game is already being played on process ID {}", self.child.unwrap());
        }
    }

    /// Checks if the current game engine already is running a game as a separate
    /// process.
    pub fn is_in_game(&self) -> bool {
        self.child.is_some()
    }

    /// Kills the currently owned game process and clears the child id.
    #[cfg(feature = "rpi")]
    pub fn kill_game(&mut self) -> bool {
        if self.is_in_game() == true {
            match std::process::Command::new("kill")
                .arg(&self.child.unwrap().to_string())
                .spawn() 
            {
                Ok(_) => self.child = None,
                Err(e) => eprintln!("error: {}", e),
            }   
        }
        true
    }
}
