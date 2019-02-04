use std::process::Command;
use std::io;

pub mod motor_controllers;
pub mod sensors;
mod sysfs_pins;
mod sysfs_pwm;

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

pub trait DigitalOutput {
    fn set_value(&mut self, val: bool);
}

pub trait AnalogOutput {
    fn set_value(&mut self, val: f32);
}