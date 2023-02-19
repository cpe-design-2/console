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

## Limitations

1. The OpenGL backend is not supported for rendering images on Linux machines for the `iced` crate. This currently prevents properly displaying PNG images for targets that choose to render with the OpenGL backend.

2. Currently, only the standard version of Godot is supported. The mono version is still under construction in getting a Godot Mono binary compiled for the targeted hardware.

Roadblocks encountered:
- extremely long compile times for the temporary Godot mono binary