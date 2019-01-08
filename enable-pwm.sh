#!/usr/bin/env bash
sudo bash -c "echo am33xx_pwm > /sys/devices/platform/bone_capemgr/slots"
sudo bash -c "echo cape-universal > /sys/devices/platform/bone_capemgr/slots"

config-pin P8_13 pwm