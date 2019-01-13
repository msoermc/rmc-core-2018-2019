#!/usr/bin/env bash
cargo doc --no-deps --release

rm -rf ./doc/

cp -r ./target/doc/ ./doc/