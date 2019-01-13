#!/usr/bin/env bash

# Set our toolchain
CROSS_COMPILER_TOOLCHAIN="armv7-unknown-linux-musleabihf" # armv7 with hardware floating-point
#CROSS_COMPILER_TOOLCHAIN="arm-unknown-linux-musleabihf" # arm with hardware floating-point
#CROSS_COMPILER_TOOLCHAIN="arm-unknown-linux-musleabi" # arm with software floating-point

REMOTE_USER="rmc"
REMOTE_PASSWORD="msoe"

# Cross compile the code
cross build --release --target=${CROSS_COMPILER_TOOLCHAIN}

# Copy the code to the home folder of the msoermc user on the other device.
# scp {src} {dest}
sudo sshpass -p ${REMOTE_PASSWORD} scp ./target/${CROSS_COMPILER_TOOLCHAIN}/release/rmc-beaglebone-core-2018-2019 ${REMOTE_USER}@192.168.7.2:/home/${REMOTE_USER}/
sudo sshpass -p ${REMOTE_PASSWORD} scp ./enable-pwm.sh ${REMOTE_USER}@192.168.7.2:/home/${REMOTE_USER}/