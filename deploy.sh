#!/usr/bin/env bash

# Set our toolchain
TOOLCHAIN="armv7-unknown-linux-musleabihf" # armv7 with hardware floating-point
#TOOLCHAIN="arm-unknown-linux-musleabihf" # arm with hardware floating-point
#TOOLCHAIN="arm-unknown-linux-musleabi" # arm with software floating-point

USER="noah"

# Cross compile the code
cross build --release --target=${TOOLCHAIN}

# Copy the code to the home folder of the msoermc user on the other device.
# scp {src} {dest}
sudo scp ./target/${TOOLCHAIN}/release/rmc-beaglebone-core-2018-2019 ${USER}@192.168.7.2:/home/${USER}/