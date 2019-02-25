#!/usr/bin/env bash

BB_IP=192.168.1.29
BB_PASSWD=msoe

cargo build
cargo build --release


scp target/debug/rmc-core rmc@${BB_IP}:~/debug
scp target/release/rmc-core rmc@${BB_IP}:~/release
scp -r ./static/ rmc@${BB_IP}:~