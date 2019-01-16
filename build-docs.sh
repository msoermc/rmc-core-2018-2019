#!/usr/bin/env bash

cargo clean
cargo doc --no-deps --document-private-items

rm -rf ./doc/

cp -r ./target/doc/ .