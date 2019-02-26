#!/usr/bin/env bash

BB_IP=192.168.1.29
BB_PASSWD='msoe'

cargo build
cargo build --release

sshpass -p ${BB_PASSWD} scp target/debug/rmc-core rmc@${BB_IP}:~/debug
sshpass -p ${BB_PASSWD} scp target/release/rmc-core rmc@${BB_IP}:~/release
sshpass -p ${BB_PASSWD} scp enable-pwm.sh rmc@${BB_IP}:~/enable-pwm.sh
sshpass -p ${BB_PASSWD} scp Rocket.toml rmc@${BB_IP}:~/Rocket.toml
sshpass -p ${BB_PASSWD} scp -r ./static/ rmc@${BB_IP}:~