#!/usr/bin/env bash

BB_IP=192.168.1.29
BB_PASSWD=msoe

cargo build
cargo build --release

rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc" target/debug/rmc-core rmc@${BB_IP}:~/debug
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc" target/release/rmc-core rmc@${BB_IP}:~/release
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc" enable-pwm.sh rmc@${BB_IP}:~/enable-pwm.sh
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc" Rocket.toml rmc@${BB_IP}:~/Rocket.toml
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc" -r ./static/ rmc@${BB_IP}:~
#rsync -P --rsh="sshpass -p $PASSWORD ssh -l me8" host.ca:/cmshome/me/file /home/me/Desktop