use std::process::Command;

pub mod analog;

pub mod digital;

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

