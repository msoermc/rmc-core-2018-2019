use std::process::Command;

use sysfs_gpio::Pin;
use sysfs_pwm::Pwm;

use crate::logging::log_data::LogData;

pub mod motor_controllers;
pub mod sensors;

/// Runs a bash script which will enable the PWM drivers and configure the pins used by the program.
pub fn enable_pins() -> Result<(), LogData> {
    let command_result = Command::new("sh").arg("enable-pwm.sh").output();

    match command_result {
        Ok(_) => Ok(()),
        Err(_) => Err(LogData::fatal("Failed to enable the pwms!")),
    }
}

/// Creates a new Pwm object from the supplied pwm chip and number.
/// On the BeagleBoneBlack, the `number` will always be `0` or `1`.
/// This corresponds to the 'A' or 'B' at the end of the pwm pinout name.
///
/// If you want info on the chip, go online, there is some info [here](https://stackoverflow.com/questions/50204329/pwm-chip-to-pin-mapping-on-beaglebone-black-v4-14).
pub fn create_pwm(chip: u32, number: u32) -> Result<Pwm, LogData> {
    match Pwm::new(chip, number) {
        Ok(pwm) => Ok(pwm),
        Err(_) => Err(LogData::fatal("Failed to create pwm!")),
    }
}

pub fn create_pin(pin: u64) -> Result<Pin, LogData> {
    Ok(Pin::new(pin))
}