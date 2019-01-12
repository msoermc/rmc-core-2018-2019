use sysfs_gpio::Pin;
use sysfs_pwm::Pwm;

use crate::logging::log_data::LogData;

pub mod motor_controllers;
pub mod sensors;

pub fn enable_pwm() {
    unimplemented!()
}

pub fn create_pwm(chip: u32, pin: u32) -> Result<Pwm, LogData> {
    match Pwm::new(chip, pin) {
        Ok(pwm) => Ok(pwm),
        Err(_) => Err(LogData::fatal("Failed to create pwm!")),
    }
}

pub fn create_pin(pin: u64) -> Result<Pin, LogData> {
    Ok(Pin::new(pin))
}