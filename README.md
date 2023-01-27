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