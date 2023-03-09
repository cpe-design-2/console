use std::error::Error;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use rppal::gpio::{OutputPin, InputPin};

// @note: Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.

/// BCM GPIO pin responsible for indicating the power-on state vs. sleep state.
const GPIO_PWR_PIN: u8 = 23;
/// BCM GPIO pin responsible for the gamestick visibility.
const GPIO_GSK_PIN: u8 = 24;

// Outputs
// -------
// - LED for indicating when the gamestick is IN/VISIBLE
// - LED to indicate power-on state vs. sleep state

// Inputs
// ------
// - button to exit a game (kill godot process)
// - button to toggle between sleep and power-on states
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
}

#[derive(Debug, PartialEq)]
pub struct Io {
    pwr_led: Pin,
    gsk_led: Pin,
}

impl Io {
    /// Mocks the struct to allow for linux machines to build with the RPI feature.
    pub fn new() -> Self {
        Self {
            pwr_led: Pin::Untied,
            gsk_led: Pin::Untied,
        }
    }

    /// Initializes the GPIO pins for corresponding input/output modes.
    pub fn configure() -> Result<Self, Box<dyn Error>> {
        println!("info: Initializing GPIO on a {} ...", DeviceInfo::new()?.model());
        // define the interface pin directions
        let mut io = Self {
            pwr_led: Pin::Output(Gpio::new()?.get(GPIO_PWR_PIN)?.into_output()),
            gsk_led: Pin::Output(Gpio::new()?.get(GPIO_GSK_PIN)?.into_output()),
        };
        // the application is running so tell the user the power is on
        io.pwr_led.set_high();
        // the application has not yet had the chance 
        io.gsk_led.set_low();
        // return the interface
        Ok(io)
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
}