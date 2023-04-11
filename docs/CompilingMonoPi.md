# Compiling Godot Mono for the Raspberry Pi 3

## About

This document outlines the numerous attempts to get a Godot Mono binary compiled for the Raspberry Pi 3 system. The plan is to virtualize the RaspberryPi system on a Mac computer with an ARM processor because RaspberryPi's also have an ARM processor.

## Progress

To get the Raspberry Pi OS running on the Mac, we first research ways other people may have done it. However, this [forum](https://forums.raspberrypi.com/viewtopic.php?t=347762) suggests that others have been unsuccessful in getting PiOS running on virtual machine for a Mac with new ARM M1/M2 processor.

Our strategy now is to use the [UTM](https://mac.getutm.app) virtualization software on the Mac to run a Linux virtualized instance to then emulate RaspberryPi OS using QEMU. UTM also utilizes QEMU under the hood.

One [forum](https://github.com/utmapp/UTM/issues/4827) suggests using QEMU by itself until UTM works for virtualizing RaspberryPi OS.

On the Linux virtualized instance, we followed this [tutorial](https://azeria-labs.com/emulate-raspberry-pi-with-qemu/) from Azeria Labs to get the RaspberryPi OS successfully running.

## Future Work

Investigate [DietPi](https://dietpi.com/docs/hardware/#utm) with UTM.

## Troubleshooting

To get Ubuntu desktop working for the ARM machine, the following links helped workaround common issues:

- https://docs.getutm.app/guides/ubuntu/
    
    Steps to install Ubuntu desktop for ARM machine for UTM (allocated 40 GB and used openSSH package).

- https://github.com/utmapp/UTM/discussions/3716
	
    Workaround for server-install issues. (Use desktop version "jammy").

- https://stackoverflow.com/questions/55043135/qemu-system-arm-redir-invalid-option
	
    Trying to get internet connection through RaspberryPi OS.
