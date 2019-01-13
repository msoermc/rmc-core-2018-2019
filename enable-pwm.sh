#!/usr/bin/env bash
sudo bash -c "echo am33xx_pwm > /sys/devices/platform/bone_capemgr/slots"
sudo bash -c "echo cape-universal > /sys/devices/platform/bone_capemgr/slots"

config-pin P8_13 pwm
config-pin P9_14 pwm
config-pin P9_16 pwm
config-pin P8_19 pwm

config-pin P9_12 gpio
config-pin P8_7 gpio
config-pin P8_9 gpio
config-pin P8_11 gpio