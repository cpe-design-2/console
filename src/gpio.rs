use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;

pub fn configure() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_low();
    thread::sleep(Duration::from_millis(500));
    pin.set_high();

    Ok(())
}


// Outputs
// -------
// - LED for indicating when the gamestick is IN/VISIBLE
// - LED to indicate power-on state vs. sleep state

// Inputs
// ------
// - button to exit a game (kill godot process)
// - button to toggle between sleep and power-on states
// - knob with resistor ladder for ADC for volume control