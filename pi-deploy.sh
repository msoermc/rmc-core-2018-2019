#!/usr/bin/env bash

BB_IP=192.168.1.29
BB_PASSWD=msoe

cargo build
cargo build --release

rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" target/debug/rmc-core rmc@${BB_IP}:~/debug
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" target/release/rmc-core rmc@${BB_IP}:~/release
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" enable-pwm.sh rmc@${BB_IP}:~/enable-pwm.sh
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" Rocket.toml rmc@${BB_IP}:~/Rocket.toml
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" -r ./static/ rmc@${BB_IP}:~/static/
#rsync -P --rsh="sshpass -p $PASSWORD ssh -l me8" host.ca:/cmshome/me/file /home/me/Desktop