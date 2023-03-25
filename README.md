# `goco`

![pipeline](https://github.com/cpe-design-2/console/actions/workflows/pipeline.yml/badge.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An open-source video game console for games built using the [Godot game engine](https://godotengine.org).

## Building the console

### Hardware

Flash a microSD card with a fresh image of the Raspberry Pi OS (64-bit) with desktop mode. The targeted device is the Raspberry Pi 3 Model B with 2GB Ram.  

View the [block diagram](./docs/goco-bd.pdf) to understand how the device and its external components combine together to create an open-source console experience.
  
The device's software will be properly initialized using the provided install script.

### Software

The goco consists of the Godot game engine built for the FRT platform and the goco binary executable. The executable is compiled in GitHub Actions for continuous deployment for the necessary target `aarch64-unknown-linux-gnu`. See the [installation script](./install.sh).


## Emulating the console

The console "operating system" application is written in the Rust programming language. It can be tested independent of its targetd Raspberry Pi platform through cross-compilation.

To run the unit tests:
```
cargo test
```

To build the application:
```
cargo build
```

To run the application:
```
cargo run
```

## Environment Variables

The following environment variables affect the console:

- `GOCO_NO_FULLSCREEN`: Disable fullscreen mode during start-up when this environment variable exists

- `GOCO_GODOT_PATH`: The complete path to the Godot engine binary to be invoked when booting a video game from a .pck file.

-  `GOCO_ROOT`: The directory from where to fetch Goco-related files. If this environment variable does not exist then it defaults to using the current working directory '.'.

## Dependencies

At a minimum, the following tools and software are required to get the application built and running:

- The Rust programming language
- Cargo package manager

## User Guide

Supported games are built using Godot version 3.5.1. The games must be specified in .pck format, and an optional icon can be set matching the same file name as the .pck file under the .png image format.

Once the Goco application is running, insert the a USB drive with under the name "GAMESTICK". The dirve is searched on the local host computer and recursively finds all the .pck files to load as the game library.

The game library can be navigated by sending 'A' keys to traverse left, and 'D' keys to traverse right. Sending a 'SPACE' key will enter the currently selected game. Sending 'E' key will eject the currently inserted "GAMESTICK" USB drive.

### Using a RasberryPi System

The Goco application is best suited to be ran on a RaspberryPi computer. An additional circuit is connected to the RaspberryPi's GPIO pins for extended functionality not available on a standard personal computer. See the configuration image [here](./docs/RPI-CIRCUIT.png) for the setup. The buttons are configured from left to right as: power button, home button, and eject button. The leftmost LED (green) signals that the system is currently in the POWER state when illuminated. The rightmost LED (yellow) signals that the system detects a USB drive as "GAMESTICK" on the current computer.

The power button has the ability to put the system into a sleep state or power state. When the green LED is illuminated, the system is in POWER state. When the LED is off, the system is in the SLEEP state.

The home button has the ability to send a subprocess command to kill the currently running Godot game engine process. This will return focus to the Goco application if the Godot engine is open. If the Godot engine is not running, there would be no effect.

The eject button has the ability to send a subprocess command to unmount the filesystem drive for the USB drive. When the yellow LED is on, the system detects the USB drive named "GAMESTICK". Pressing the button will remove the drive from the filesystem and set the yellow LED to off.

## Limitations

1. The OpenGL backend is not supported for rendering images on Linux machines for the `iced` crate. This currently prevents properly displaying PNG images for targets that choose to render with the OpenGL backend.
 
Roadblocks encountered:
- failed to compile Vulkan (alternate WGPU_BACKEND) for RaspberryPi OS. See these projects and forums: [rpi-vk-driver/issues](https://github.com/Yours3lf/rpi-vk-driver/issues/6), [vulkan-loader](https://github.com/KhronosGroup/Vulkan-Loader), [vulkan-header](https://github.com/KhronosGroup/Vulkan-Headers/blob/main/BUILD.md).

2. Currently, only the standard version of Godot is supported. The mono version is still under construction in getting a Godot Mono binary compiled for the targeted hardware.

Roadblocks encountered:
- extremely long compile times for the temporary Godot mono binary