#!/usr/bin/env bash

rm -rf ./test-release
rm -rf ./test-debug

cargo build
cargo build --release
cargo build --tests --release -Z unstable-options --out-dir ./test-release
cargo build --tests -Z unstable-options --out-dir ./test-debug