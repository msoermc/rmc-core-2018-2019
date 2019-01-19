use std::process::Command;

use sysfs_gpio::Pin;
use sysfs_pwm::Pwm;

pub mod motor_controllers;
pub mod sensors;

/// Runs a bash script which will enable the PWM drivers and configure the pins used by the program.
pub fn enable_pins() -> Result<(), ()> {
    let command_result = Command::new("sh").arg("enable-pwm.sh").output();

    match command_result {
        Ok(_) => Ok(()),
        Err(_) => {
            error!("Failed to enable the pwms!");
            Err(())
        },
    }
}