use crate::env;
use crate::game::Game;

#[derive(Debug, PartialEq)]
pub struct Engine {
    exe: String,
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
        }
    }

    /// Invokes the Godot game engine and starts a [Game] `game`.
    pub fn play_game(&self, game: &Game) -> () {
        match std::process::Command::new(&self.exe)
            .arg("--fullscreen")
            .arg("--always-on-top")
            .arg("--main-pack")
            .arg(game.get_pck())
            .spawn()
        {
            Ok(_) => (),
            Err(_) => panic!("failed to load game"),
        }
    }
}
