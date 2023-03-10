mod engine;
mod env;
mod game;
mod gamestick;
mod os;

#[cfg(feature = "rpi")]
mod gpio;

use os::Os;
use iced::Application;
use iced::Settings;

pub fn go() -> u8 {
    println!("info: Booting up GOCO ...");
    match Os::run(Settings {
            exit_on_close_request: false,
            ..Settings::default()
        }) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("error: {}", e);
                101
            }
    }
}