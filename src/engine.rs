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
        #[cfg(not(feature = "rpi"))]
        {
            // check if the PID still exists (external event may have quit GODOT)
            if self.is_in_game() == true && self.check_proc_id_alive() == false {
                self.child = None;
            } 
        }
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

    /// Runs a subprocess to check the currently running processes if the Godot
    /// application is still actively running.
    /// 
    /// This function is only required for targets not built for the RaspberryPi.
    #[cfg(not(feature = "rpi"))]
    fn check_proc_id_alive(&mut self) -> bool {
        // allow multiple instances of Godot on windows because of unavailable 'ps' command
        if cfg!(target_os = "windows") == true {
            return false;
        }

        if let Some(id) = self.child {
            match std::process::Command::new("ps")
                .arg("-p")
                .arg(id.to_string())
                .output()
            {
                // store the child's ID for future usage
                Ok(rc) => {
                    if rc.status.success() == true {
                        // read the output string
                        let text = String::from_utf8(rc.stdout).unwrap();
                        // the process is being idle (essentially "killed")
                        if text.contains("0:00.00") == true {
                            false
                        // the process is actively running
                        } else {
                            true
                        }
                    // the PID is not available to be found (essentially "killed")
                    } else {
                        false
                    }
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                    false
                },
            }
        } else {
            false
        }
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
