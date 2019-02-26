#!/usr/bin/env bash

BB_IP=192.168.1.29
BB_PASSWD='msoe'

cargo build
cargo build --release


sudo sshpass -p ${BB_PASSWD} scp target/debug/rmc-core rmc@${BB_IP}:~/debug -o StrictHostKeyChecking=no
sudo sshpass -p ${BB_PASSWD} scp target/release/rmc-core rmc@${BB_IP}:~/release -o StrictHostKeyChecking=no
sudo sshpass -p ${BB_PASSWD} scp enable-pwm.sh rmc@${BB_IP}:~/enable-pwm.sh -o StrictHostKeyChecking=no
sudo sshpass -p ${BB_PASSWD} scp Rocket.toml rmc@${BB_IP}:~/Rocket.toml -o StrictHostKeyChecking=no
sudo sshpass -p ${BB_PASSWD} scp -r ./static/ rmc@${BB_IP}:~ -o StrictHostKeyChecking=no