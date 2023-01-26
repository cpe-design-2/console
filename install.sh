#!/bin/sh
# File: install.sh
# Author: Chase Ruskin
# Details:
#   This file contains the entire installation required for setting up the open-
#   source video game console 'goco'.
#
#   This installation assumes the user is using a newly configured Pi with the
#   provided specifications.
#
#   Hardware: Raspberry Pi 3 Model B V1.2
#   Storage: 16GB microSD card
#   Operating System: Raspberry Pi OS (64-bit) [Released: 2022-09-22]
#
#   Download this script and execute it:
#
#   $   sh ./install.sh
#   
# References:
#   - https://github.com/efornara/frt/blob/2.0.1/doc/Compile.md
#   - https://linuxhint.com/use-etc-rc-local-boot/
#

# stop installation on any intermediate error
set -e

# 1) COMPILE GODOT GAME ENGINE
# ----------------------------

GODOT_VERSION="3.5.1-stable"

FRT_VERSION="2.0"

# install dependencis for godot and goco
sudo apt-get install git build-essential scons pkg-config clang llvm lld libsdl2-dev libgles2-mesa-dev libfontconfig1

# download the zipped source code
curl -LO https://github.com/godotengine/godot/archive/refs/tags/$GODOT_VERSION.zip
# unzip the archive
tar -xf $GODOT_VERSION.zip
# remove the compressed archive
rm $GODOT_VERSION.zip

# add the FRT paltform for the RPi3
cd ./godot-$GODOT_VERSION/platform
git clone -b $FRT_VERSION https://github.com/efornara/frt
# return to godot's base folder
cd ..

# execute the build process
scons platform=frt tools=no target=release use_llvm=yes -j 4 module_webm_enabled=no

# place the compiled executable in a known path
# binary -> ./bin/godot.frt.opt.llvm


# 2) UPDATE .BASHRC


# 3) DOWNLOAD THE GOCO COMPILED EXECUTABLE
# ----------------------------------------


# 4) EDIT "/etc/rc.local" TO ALLOW GOCO PROGRAM TO START-UP
# ---------------------------------------------------------