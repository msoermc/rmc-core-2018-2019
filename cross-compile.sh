#!/usr/bin/env bash
CROSS_COMPILER_TOOLCHAIN="armv7-unknown-linux-musleabihf" # armv7 with hardware floating-point
SYSROOT=/build/root

export PKG_CONFIG_DIR=
export PKG_CONFIG_LIBDIR=${SYSROOT}/usr/lib/pkgconfig:${SYSROOT}/usr/share/pkgconfig
export PKG_CONFIG_SYSROOT_DIR=${SYSROOT}
export PKG_CONFIG_ALLOW_CROSS=1

# Cross compile the code
cross build --release --target=${CROSS_COMPILER_TOOLCHAIN}