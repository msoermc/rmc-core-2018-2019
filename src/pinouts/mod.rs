use std::process::Command;

/// Contains all code for "analog" pinout signals, including things that only simulate an analog signal like
/// PWM.
pub mod analog;

/// Contains all code for digital pinout.
pub mod digital;

/// Contains factory functions for producing the pinouts used by the robot.
pub mod factories;

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

