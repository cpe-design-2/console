use iced::event::Event;
use iced::executor;
use iced::keyboard::KeyCode;
use iced::subscription;
use iced::window;
use std::time::{Duration, Instant};
use iced::time;
use iced::keyboard::Event::KeyPressed;
use iced::{Alignment, Application, Command, Element, Length, Subscription, Theme};
use iced::widget::{button, Container};
use iced::widget::text;

use crate::engine::Engine;
use crate::game::Game;
use crate::gamestick::GameStick;
use crate::animator::Animation;

#[cfg(feature = "rpi")]
use crate::gpio::Io;

// model the state of the application
#[derive(Debug, PartialEq)]
pub struct Os {
    /// The backend Godot game engine to invoke for playing games.
    engine: Engine,
    /// The available media drive.
    drive: GameStick,
    /// List of all available loaded games.
    library: Vec<Game>,
    /// Determine the current selected game in the list.
    count: usize,
    /// Track the application's state.
    state: State,
    /// Store the state of the IO interface.
    #[cfg(feature = "rpi")]
    io: Io,
    /// Create an attribute for the animation player
    insert_animation: Animation,
}

#[derive(Debug, PartialEq)]
enum State {
    /// Request the user to insert a game drive.
    Requesting,
    /// Read games from the game drive.
    Loading,
}

impl Os {
    /// Constructs a new [Os] structure.
    /// 
    /// Also immediately checks if a [GameStick] is entered to load games without initial delay.
    pub fn new() -> Self {
        // configure the Pi's IO
        #[cfg(feature = "rpi")]
        let io = match Io::configure() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("error: {}", e);
                Io::new()
            }
        };

        let mut os = Self {
            engine: Engine::new(),
            drive: GameStick::new(),
            library: Vec::new(),
            count: 0,
            state: State::Requesting,
            insert_animation: Animation::new(),
            #[cfg(feature = "rpi")]
            io: io,
        };
        // check if game stick is already inserted on application start
        if os.drive.exists() == true {
            os.initialize_library();
        }
        
        os
    }

    /// Access the games surrounding the current index.
    /// 
    /// Returns an element with entry `None` if the index is out of bounds of the
    /// currently loaded game library.
    fn get_nearby_games(&self) -> [Option<&Game>; 3] {
        let mut result = [None, None, None];
        // check at one level before and one level after the current index
        result[0] = if self.count == 0 {
            None
        } else {
            self.library.get(self.count - 1)
        };

        result[1] = self.library.get(self.count);

        result[2] = self.library.get(self.count + 1);

        result
    }

    /// Increments the library's index by 1 only if that index value exists (< length).
    /// 
    /// Returns `true` if the count was successfully incremented.
    fn shift_shelf_right(&mut self) -> bool {
        // cap at len()-1
        let able_to_shift = self.count + 1 < self.library.len();
        if able_to_shift == true {
            self.count += 1;
        }
        able_to_shift
    }

    /// Decrements the library's index by 1 only if that index value exists (>= 1).
    /// 
    /// Returns `true` if the count was successfully decremented.
    fn shift_shelf_left(&mut self) -> bool {
        let able_to_shift = self.count >= 1;
        // cap at 0
        if able_to_shift == true {
            self.count -= 1;
        }
        able_to_shift
    }

    /// Transitions from the `Requesting` state to the `Loading` State while
    /// if the drive can successfully be read from the filesystem.
    /// 
    /// Returns `true` if the state transition occurs successfully and `false` otherwise.
    fn initialize_library(&mut self) -> bool {
        self.count = 0;
        if self.drive.can_read_dir() == false { return false };
        self.library = GameStick::load(self.drive.get_path());
        self.state = State::Loading;
        true
    }

    /// Transitions from the `Loading` state to the `Requesting` State while
    /// unloading the game library.
    /// 
    /// Returns `true` if the state transition occurs successfully and `false` otherwise.
    fn flush_library(&mut self) -> bool {
        self.library = Vec::new();
        self.count = 0;
        self.state = State::Requesting;
        true
    }

    /// Safely ejects the USB GAMESTICK drive and cleans up the currently
    /// loaded library.
    fn remove_drive(&mut self) -> bool {
        if self.drive.eject() == true {
            self.flush_library()
        } else {
            false
        }
    }

    /// Triggers the underlying operating system into 'suspend' mode. Before
    /// issuing the command, the power LED will be turned off. If the command
    /// fails, then the power LED status will be restored.
    /// 
    /// Suspend stops operation of all applications and puts the machine into 
    /// a low-power mode. Various triggers can resume the machine, among them 
    /// pressing a key or quickly pressing and releasing the power button.
    #[cfg(feature = "rpi")]
    fn power_down(&mut self) -> bool {
        self.io.disable_pwr_led();
        match std::process::Command::new("shutdown")
            .arg("-h")
            .arg("now")
            .spawn()
        {
            Ok(_) => true,
            // restore the power LED status if the command failed
            Err(e) => {
                eprintln!("error: {}", e);
                self.io.enable_pwr_led();
                false
            },
        }
    }

    /// Invokes a command to quit the godot game engine process to essentially
    /// "return home".
    #[cfg(feature = "rpi")]
    fn quit_game(&mut self) -> bool {
        self.engine.kill_game()
    }

    /// Invokes the engine to run the game at index `count` in the loaded game library.
    /// 
    /// The Godot game engine is called to spawn a new process.
    fn select_game(&mut self) {
        // guaranteed to have `count` as a valid index for game library vector
        self.engine.play_game(self.library.get(self.count).unwrap());
    }

    /// Checks if the gamestick is available on the filesystem and changes the
    /// pin's level accordingly.
    /// 
    /// - Gamestick filesystem exists: LED = `on`
    /// - Gamestick filesytem does not exist: LED = `off`
    #[cfg(feature = "rpi")]
    fn update_gamestick_led(&mut self) {
        match self.drive.exists() {
            true => self.io.enable_gsk_led(),
            false => self.io.disable_gsk_led(),
        }
    }

    /// Checks if the current system is in power-saving mode and changes the
    /// pin's level accordingly.
    /// 
    /// - Full-power mode: LED = `on`
    /// - Low-power mode: LED = `off`
    #[cfg(feature = "rpi")]
    fn update_power_led(&mut self) {
        // if the application is running then the power pin must be enabled
        self.io.enable_pwr_led();
    }

    /// Reads the stored data for the input buttons to call certain functionality.
    #[cfg(feature = "rpi")]
    fn update_inputs(&mut self) {
        // remove the drive from the filesystem
        if self.io.check_eject_triggered() == true {
            let _ = self.remove_drive();
        }
        // return to the home screen (quit Godot process)
        if self.io.check_home_triggered() == true {
            let _ = self.quit_game();
        }
        // send the system in sleep state
        if self.io.check_power_triggered() == true {
            let _ = self.power_down();
        }
    }
}

// define the possible user interactions of the main screen operating system
#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    ScanDrive(Instant),
    PlayGame,
    UpdateIo(Instant),
}

impl Application for Os {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Os, Command<Message>) {
        (
            Os::new(),
            // determine at run-time the full-screen mode
            match std::env::var(crate::env::GOCO_NO_FULLSCREEN) {
                Ok(_) => Command::none(),
                Err(_) => iced::window::change_mode::<Message>(window::Mode::Fullscreen),
            },
        )
    }

    fn title(&self) -> String {
        String::from("GOCO")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // handle background checking the filesystem for the gamestick directory
            Message::ScanDrive(_instant) => {
                match self.state {
                    State::Requesting => {
                        // attempt to load the gamestick's library
                        if self.drive.exists() == true { 
                            println!("info: GAMESTICK detected ...");
                            self.initialize_library();
                        } else {
                            println!("info: Scanning for GAMESTICK at directory: {:?}", self.drive.get_path());
                            self.insert_animation.next();
                        }
                    }
                    State::Loading => {
                        // attempt to remove the gamestick's library (USB media is gone)
                        if self.drive.exists() == false {
                            println!("info: Removing GAMESTICK ...");
                            self.flush_library();
                        }
                    }
                }
                Command::none()
            }
            // handle event to enter a game
            Message::PlayGame => {
                self.select_game();
                Command::none()
            }
            // handle keyboard input (only subscribed during `Loading` state)
            Message::EventOccurred(Event::Keyboard(event)) => {
                match self.state {
                    State::Loading => {
                        if let KeyPressed { key_code, modifiers: _ } = event {
                            match key_code {
                                // right
                                KeyCode::D => { self.shift_shelf_right(); },
                                // left
                                KeyCode::A => { self.shift_shelf_left(); },
                                // down
                                KeyCode::S => { },
                                // up
                                KeyCode::W => { },
                                // action key (spacebar)
                                KeyCode::Space => { self.select_game(); }
                                // allow the user to eject the drive using the 'E' key
                                #[cfg(not(feature = "rpi"))]
                                KeyCode::E => { self.remove_drive(); }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
                Command::none()
            }
            // handle window closing
            Message::EventOccurred(Event::Window(window::Event::CloseRequested)) => {
                window::close()
            }
            Message::EventOccurred(_) => {
                Command::none()
            }
            // handle updating IO pins
            Message::UpdateIo(_) => {
                #[cfg(feature = "rpi")]
                {
                    println!("info: Refreshing IO ...");
                    self.update_gamestick_led();
                    self.update_power_led();
                    self.update_inputs();
                }
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            subscription::events().map(Message::EventOccurred),
            time::every(Duration::from_millis(500)).map(Message::UpdateIo),
            time::every(Duration::from_millis(1000)).map(Message::ScanDrive),
        ])
    }

    fn view(&self) -> Element<Message> {
        match self.state {
            State::Requesting => {
                iced::widget::column![
                    text(self.insert_animation.get_text())
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                    self.insert_animation.draw()
                ]
                .padding(128)
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(20)
                .align_items(Alignment::Center)
                .into()
            },
            State::Loading => {
                let nearby_games = self.get_nearby_games();
                // use a column: a simple vertical layout
                iced::widget::column![
                    // display the game's in a row      
                    iced::widget::row![
                        // game appear on the LHS
                        match nearby_games[0] { Some(g) => { Container::new(g.draw(false)) } None => { Container::new(Game::blank()) } },
                        // the middle index (`1`) is the selected game
                        match nearby_games[1] { Some(g) => { Container::new(g.draw(true)) } None => { Container::new(Game::blank()) } },
                        // game appear on the RHS
                        match nearby_games[2] { Some(g) => { Container::new(g.draw(false)) } None => { Container::new(Game::blank()) } },
                    ]
                    .spacing(64),
                    button("PLAY").on_press(Message::PlayGame),
                ]
                .padding(32)
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(64)
                .align_items(Alignment::Center)
                .into()
            },
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn it_load_nearby_games() {
        let mut os = Os {
            drive: GameStick::new(),
            insert_animation: Animation::new(),
            state: State::Requesting,
            engine: Engine::new(),
            library: GameStick::load(&PathBuf::from(format!("{}/testenv/GAMESTICK", env!("CARGO_MANIFEST_DIR")))),
            count: 0,
            #[cfg(feature = "rpi")]
            io: Io::new(),
        };
        {
            let shelf = os.get_nearby_games();
            assert_eq!(shelf[0].is_none(), true);
            assert_eq!(shelf[1].unwrap(), &os.library[0]);
            assert_eq!(shelf.iter().filter(|p| p.is_some()).count(), 2);
        }
        os.shift_shelf_right();
        {
            let shelf = os.get_nearby_games();
            assert_eq!(shelf[0].unwrap(), &os.library[0]);
            assert_eq!(shelf[1].unwrap(), &os.library[1]);
            assert_eq!(shelf.iter().filter(|p| p.is_some()).count(), 3);
        }
        os.shift_shelf_left();
        {
            let shelf = os.get_nearby_games();
            assert_eq!(shelf[0].is_none(), true);
            assert_eq!(shelf[1].unwrap(), &os.library[0]);
            assert_eq!(shelf.iter().filter(|p| p.is_some()).count(), 2);
        }
        os.shift_shelf_left();
        {
            let shelf = os.get_nearby_games();
            assert_eq!(shelf[0].is_none(), true);
            assert_eq!(shelf[1].unwrap(), &os.library[0]);
            assert_eq!(shelf.iter().filter(|p| p.is_some()).count(), 2);
        }
    }
}