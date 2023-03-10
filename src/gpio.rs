use std::error::Error;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use rppal::gpio::{OutputPin, InputPin, Trigger, Level};

use std::sync::atomic::{AtomicBool, Ordering};

// @note: Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.

// input pins

/// BCM GPIO pin responsible for toggling between sleep and power-on state (power button).
/// 
/// Note the power button must be connected to GPIO3 (pin 5- SCL line).
/// 
/// Source: https://howchoo.com/g/mwnlytk3zmm/how-to-add-a-power-button-to-your-raspberry-pi
const GPIO_POWER_BTN: u8 = 3;
/// BCM GPIO pin responsible for killing the godot process (home button).
const GPIO_HOME_BTN: u8 = 27;
/// BCM GPIO pin responsible for ejecting the gamestick (eject button).
const GPIO_EJECT_BTN: u8 = 25;

// outputs pins

/// BCM GPIO pin responsible for indicating the power-on state vs. sleep state.
const GPIO_PWR_PIN: u8 = 23;
/// BCM GPIO pin responsible for the gamestick visibility.
const GPIO_GSK_PIN: u8 = 24;

// global statics to be handled by the asynchronous button inputs and the main goco process
static IS_HOME_TRIGGERED: AtomicBool = AtomicBool::new(false);
static IS_POWER_TRIGGERED: AtomicBool = AtomicBool::new(false);
static IS_EJECT_TRIGGERED: AtomicBool = AtomicBool::new(false);

// Outputs
// -------
// - LED for indicating when the gamestick is IN/VISIBLE
// - LED to indicate power-on state vs. sleep state

// Inputs
// ------
// - button to exit a game (kill godot process) (HOME BUTTON)
// - button to toggle between sleep and power-on states (POWER BUTTON)
// - button to eject gamestick (EJECT BUTTON)
// - knob with resistor ladder for ADC for volume control

/// Abstraction layer to allow for pins to be untied/unused in code without causing
/// errors.
#[derive(Debug, PartialEq)]
enum Pin {
    Output(OutputPin),
    Input(InputPin),
    Untied,
}

impl Pin {
    /// Sets a pin configured as output to high status. 
    /// 
    /// All other pin configurations are left unchanged.
    fn set_high(&mut self) {
        match self {
            Self::Output(p) => p.set_high(),
            _ => (),
        }
    }

    /// Sets a pin configured as output to low status.    
    /// 
    /// All other pin configurations are left unchanged.
    fn set_low(&mut self) {
        match self {
            Self::Output(p) => p.set_low(),
            _ => (),
        }
    }

    /// Accesses the internal [InputPin] of the Pin. Panics if used on a pin
    /// not configured as an input.
    fn as_input_pin_mut(&mut self) -> &mut InputPin {
        match self {
            Self::Input(p) => p,
            _ => panic!("pin {:?} cannot be accessed as input pin", self),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Io {
    pwr_led: Pin,
    gsk_led: Pin,
    eject_btn: Pin,
    home_btn: Pin,
    power_btn: Pin,
}

impl Io {
    /// Mocks the struct to allow for linux machines to build with the RPI feature.
    pub fn new() -> Self {
        Self {
            pwr_led: Pin::Untied,
            gsk_led: Pin::Untied,
            eject_btn: Pin::Untied,
            home_btn: Pin::Untied,
            power_btn: Pin::Untied,
        }
    }

    /// Initializes the GPIO pins for corresponding input/output modes.
    pub fn configure() -> Result<Self, Box<dyn Error>> {
        println!("info: Initializing GPIO on a {} ...", DeviceInfo::new()?.model());
        // define the interface pin directions
        let mut io = Self {
            pwr_led: Pin::Output(Gpio::new()?.get(GPIO_PWR_PIN)?.into_output()),
            gsk_led: Pin::Output(Gpio::new()?.get(GPIO_GSK_PIN)?.into_output()),
            // configure the button inputs with the internal pull-up resistors
            eject_btn: Pin::Input(Gpio::new()?.get(GPIO_EJECT_BTN)?.into_input_pullup()),
            home_btn: Pin::Input(Gpio::new()?.get(GPIO_HOME_BTN)?.into_input_pullup()),
            power_btn: Pin::Input(Gpio::new()?.get(GPIO_POWER_BTN)?.into_input_pullup()),
        };
        
        // the application is running so tell the user the power is on
        io.pwr_led.set_high();
        // the application has not yet had the chance 
        io.gsk_led.set_low();

        // set the asynchronous interrupts for input pins
        io.eject_btn.as_input_pin_mut().set_async_interrupt(Trigger::FallingEdge, Self::eject_callback)?;
        io.home_btn.as_input_pin_mut().set_async_interrupt(Trigger::FallingEdge, Self::home_callback)?;
        io.power_btn.as_input_pin_mut().set_async_interrupt(Trigger::FallingEdge, Self::power_callback)?;

        // return the interface
        Ok(io)
    }

    /// Sets the atomic variable to `true` for a eject button press.
    fn eject_callback(level: Level) -> () {
        if level == Level::Low {
            IS_EJECT_TRIGGERED.store(true, Ordering::SeqCst);
            // println!("eject button pressed!");
        }
    }

    /// Sets the atomic variable to `true` for a power button press.
    fn power_callback(level: Level) -> () {
        if level == Level::Low {
            IS_POWER_TRIGGERED.store(true, Ordering::SeqCst);
            // println!("power button pressed!");
        }
    }

    /// Sets the atomic variable to `true` for a home button press.
    fn home_callback(level: Level) -> () {
        if level == Level::Low {
            IS_HOME_TRIGGERED.store(true, Ordering::SeqCst);
            // println!("home button pressed!");
        }
    }

    /// Sets the Gamestick visibility LED to `on`.
    pub fn enable_gsk_led(&mut self) {
        self.gsk_led.set_high();
    }

    /// Sets the Gamestick visibility LED to `off`.
    pub fn disable_gsk_led(&mut self) {
        self.gsk_led.set_low();
    }

    /// Sets the power state LED to `on`.
    pub fn enable_pwr_led(&mut self) {
        self.pwr_led.set_high();
    }

    /// Sets the power state LED to `off`.
    pub fn disable_pwr_led(&mut self) {
        self.pwr_led.set_low();
    }

    /// Returns the value stored in the atomic variable and clears it if set.
    pub fn check_power_triggered(&self) -> bool {
        let result: bool = IS_POWER_TRIGGERED.load(Ordering::SeqCst);
        // clear the flag if it was raised (unhandled)
        if result == true {
            IS_POWER_TRIGGERED.store(false, Ordering::SeqCst);
        }
        result
    }

    /// Returns the value stored in the atomic variable and clears it if set.
    pub fn check_eject_triggered(&self) -> bool {
        let result: bool = IS_EJECT_TRIGGERED.load(Ordering::SeqCst);
        // clear the flag if it was raised (unhandled)
        if result == true {
            IS_EJECT_TRIGGERED.store(false, Ordering::SeqCst);
        }
        result
    }

    /// Returns the value stored in the atomic variable and clears it if set.
    pub fn check_home_triggered(&self) -> bool {
        let result: bool = IS_HOME_TRIGGERED.load(Ordering::SeqCst);
        // clear the flag if it was raised (unhandled)
        if result == true {
            IS_HOME_TRIGGERED.store(false, Ordering::SeqCst);
        }
        result
    }
}