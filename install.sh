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

GOCO_ROOT="$HOME/GOCO"

GOCO_VERSION="0.1.0"

GOCO_ARTIFACT="goco-$GOCO_VERSION-aarch64-linux"

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

# create directory to store GOCO related contents
mkdir -p $GOCO_ROOT

# copy the compiled executable to a known path
mkdir -p $GOCO_ROOT/bin
cp ./bin/godot.frt.opt.llvm $GOCO_ROOT/bin/godot.frt.opt.llvm 


# 2) UPDATE .BASHRC
# -----------------
echo "export GOCO_GODOT_PATH=\"$GOCO_ROOT/bin/godot.frt.opt.llvm\"" >> $HOME/.bashrc
echo "export GOCO_ROOT=\"$GOCO_ROOT\"" >> $HOME/.bashrc


# 3) DOWNLOAD THE GOCO COMPILED EXECUTABLE
# ----------------------------------------
curl -LO https://github.com/cpe-design-2/console/releases/download/$GOCO_VERSION/$GOCO_ARTIFACT.zip
tar -xf $GOCO_ARTIFACT.zip
# allow the goco binary to be executed
chmod +x ./$GOCO_ARTIFACT/bin/goco
# move the binary to a known path
cp ./$GOCO_ARTIFACT/bin/goco $GOCO_ROOT/bin/goco
# recursively copy all assets to assets folder found at known location
cp -R ./$GOCO_ARTIFACT/assets $GOCO_ROOT/assets


# 4) EDIT "/etc/rc.local" TO ALLOW GOCO PROGRAM TO START-UP
# ---------------------------------------------------------