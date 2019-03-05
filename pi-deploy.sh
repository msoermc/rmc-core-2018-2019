#!/usr/bin/env bash

BB_IP=192.168.1.29
BB_PASSWD=msoe
ROBOT_DIR=~/robot

rm -rf ./test-release
rm -rf ./test-debug

cargo build
cargo build --release
cargo build --tests --release -Z unstable-options --out-dir ./test-release
cargo build --tests -Z unstable-options --out-dir ./test-debug

rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" target/debug/rmc-core rmc@${BB_IP}:${ROBOT_DIR}/debug
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" target/release/rmc-core rmc@${BB_IP}:${ROBOT_DIR}/release
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" enable-pwm.sh rmc@${BB_IP}:${ROBOT_DIR}/enable-pwm.sh
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" Rocket.toml rmc@${BB_IP}:${ROBOT_DIR}/Rocket.toml
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" -r ./static/ rmc@${BB_IP}:${ROBOT_DIR}/static/

rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" -r ./test-release/ rmc@${BB_IP}:${ROBOT_DIR}/test-release/
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" -r ./test-debug/ rmc@${BB_IP}:${ROBOT_DIR}/test-debug/