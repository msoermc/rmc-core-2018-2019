#!/usr/bin/env bash

TOOLCHAIN="armv7-unknown-linux-musleabihf"
#TOOLCHAIN="arm-unknown-linux-musleabihf"
#TOOLCHAIN="arm-unknown-linux-musleabi"

# Cross compile the code
cross build --release --target=${TOOLCHAIN}

# Copy the code to the home folder of the msoermc user on the other device.
# scp {src} {dest}
sudo scp ./target/${TOOLCHAIN}/release/rmc-beaglebone-core-2018-2019 rmc@192.168.7.2:~