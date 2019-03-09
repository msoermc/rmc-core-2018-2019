#!/usr/bin/env bash
PASSWORD="msoe"
echo ${PASSWORD} | sudo -S bash -c "echo am33xx_pwm > /sys/devices/platform/bone_capemgr/slots"
echo ${PASSWORD} | sudo -S bash -c "echo cape-universal > /sys/devices/platform/bone_capemgr/slots"

config-pin P8_13 pwm
config-pin P9_14 pwm
config-pin P9_16 pwm
config-pin P8_19 pwm
config-pin P9_22 pwm
config-pin P9_21 pwm
config-pin P9_42 pwm
config-pin P9_28 pwm

config-pin P9_12 gpio
config-pin P8_9 gpio
config-pin P8_17 gpio
config-pin P8_11 gpio

config-pin P9_15 gpio
config-pin P9_30 gpio
config-pin P8_7 gpio
config-pin P8_8 gpio
config-pin P8_15 gpio
config-pin P8_16 gpio