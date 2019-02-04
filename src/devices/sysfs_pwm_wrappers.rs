use std::io;
use std::process;

use sysfs_pwm;

use super::AnalogOutput;

const PERIOD_NS: u32 = 20_000;

pub struct SysfsPwm {
    pwm: sysfs_pwm::Pwm,
    chip: u32,
    number: u32,
}

impl AnalogOutput for SysfsPwm {
    fn set_value(&mut self, val: f32) -> Result<(), String> {
        let pwm_out = f32::abs(val * PERIOD_NS as f32) as u32;

        if self.pwm.set_duty_cycle_ns(pwm_out).is_err() {
            warn!("Failed to set duty cycle for sysfs pwm <{},{}>!", self.chip, self.number);
            if let Err(e) = self.pwm.export() {
                warn!("Failed to reexport sysfs pwm <{},{}>!", self.chip, self.number);
                Err(e.to_string())
            } else {
                if let Err(e) = self.pwm.set_period_ns(PERIOD_NS) {
                    warn!("Failed to export sysfs pwm <{},{}>!", self.chip, self.number);
                    Err(e.to_string())
                } else {
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }
}

impl SysfsPwm {
    pub fn create(chip: u32, number: u32, board_location: &str) -> sysfs_pwm::Result<Self> {
        let config_command = process::Command::new("config-pin")
            .arg(board_location)
            .arg("pwm")
            .output();

        if let Err(e) = config__command {
            warn!("Failed to configure pin {}! Error:\n{}", board_location, e);
            return Err(sysfs_pwm::Error::Unexpected(format!("Failed to configure pin {}! Error:\n{}", board_location, e)));
        }

        let pwm = sysfs_pwm::Pwm::new(chip, number)?;

        if pwm.export().is_err() {
            error!("Failed to export sysfs pwm <{},{}>!", chip, number);
        }

        if pwm.set_period_ns(PERIOD_NS).is_err() {
            error!("Failed to export sysfs pwm <{},{}>!", chip, number);
        }

        Ok(Self {
            pwm,
            chip,
            number,
        })
    }
}