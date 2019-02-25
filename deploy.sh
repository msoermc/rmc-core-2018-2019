#!/usr/bin/env bash

# Set our toolchain
CROSS_COMPILER_TOOLCHAIN="armv7-unknown-linux-musleabihf" # armv7 with hardware floating-point
#CROSS_COMPILER_TOOLCHAIN="arm-unknown-linux-musleabihf" # arm with hardware floating-point
#CROSS_COMPILER_TOOLCHAIN="arm-unknown-linux-musleabi" # arm with software floating-point

REMOTE_USER="rmc"
REMOTE_PASSWORD="msoe"

SYSROOT=/build/root

export PKG_CONFIG_DIR=
export PKG_CONFIG_LIBDIR=${SYSROOT}/usr/lib/pkgconfig:${SYSROOT}/usr/share/pkgconfig
export PKG_CONFIG_SYSROOT_DIR=${SYSROOT}
export PKG_CONFIG_ALLOW_CROSS=1

# Cross compile the code
cross build --release --target=${CROSS_COMPILER_TOOLCHAIN}

# Copy the code to the home folder of the msoermc user on the other device.
# scp {src} {dest}
sudo sshpass -p ${REMOTE_PASSWORD} scp ./target/${CROSS_COMPILER_TOOLCHAIN}/release/rmc-core ${REMOTE_USER}@192.168.7.2:/home/${REMOTE_USER}/
sudo sshpass -p ${REMOTE_PASSWORD} scp ./enable-pwm.sh ${REMOTE_USER}@192.168.7.2:/home/${REMOTE_USER}/
sudo sshpass -p ${REMOTE_PASSWORD} scp -R ./static/ ${REMOTE_USER}@192.168.7.2:/home/${REMOTE_USER}/