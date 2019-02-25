#!/usr/bin/env bash

BB_IP=192.168.1.29
BB_PASSWD=msoe

cargo build
cargo build --release


sudo sshpass -p ${BB_PASSWD} scp target/debug/rmc-core rmc@${BB_IP}:~/debug
sudo sshpass -p ${BB_PASSWD} scp target/release/rmc-core rmc@${BB_IP}:~/release
sudo sshpass -p ${BB_PASSWD} scp -r ./static/ rmc@${BB_IP}:~