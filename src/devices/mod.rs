use std::io;
use std::process::Command;

pub mod motor_controllers;
pub mod sensors;
pub mod sysfs_pin_wrappers;
pub mod sysfs_pwm_wrappers;

/// Runs a bash script which will enable the PWM drivers and configure the pins used by the program.
pub fn enable_pins() -> Result<(), ()> {
    let command_result = Command::new("sh").arg("enable-pwm.sh").output();

    match command_result {
        Ok(_) => Ok(()),
        Err(_) => {
            error!("Failed to enable the pwms!");
            Err(())
        }
    }
}

pub trait DigitalOutput: Send {
    fn set_value(&mut self, val: bool) -> Result<(), String>;
}

pub trait AnalogOutput: Send {
    fn set_value(&mut self, val: f32) -> Result<(), String>;
}