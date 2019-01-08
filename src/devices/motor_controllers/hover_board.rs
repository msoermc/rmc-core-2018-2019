use super::MotorController;
use sysfs_pwm::{Pwm};
use sysfs_gpio::Pin;
use crate::logging::log_data::LogData;

const PERIOD_NS: u32 = 20_000;

pub struct HoverBoardMotor {
    is_inverted: bool,
    pwm: Pwm,
    direction: Pin,
}

impl MotorController for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), LogData> {
        if self.pwm.with_exported(|| {
            let duty_cycle = new_speed * (PERIOD_NS as f32);
            self.pwm.set_duty_cycle_ns(duty_cycle as u32)
        }).is_err() {
            Err(LogData::error("Problem changing the duty cycle in MC!"))
        } else {
            let is_negative = new_speed < 0.0;

            let new_direction = is_negative ^ self.is_inverted;

            if self.direction.with_exported(|| {
                self.direction.set_active_low(new_direction)
            }).is_err() {
                Err(LogData::error("Problem changing the direction in MC!"))
            } else {
                Ok(())
            }
        }
    }

    fn stop(&mut self) -> Result<(), LogData> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) -> Result<(), LogData> {
        self.is_inverted = !self.is_inverted;

        Ok(())
    }

    fn is_inverted(&self) -> Result<bool, LogData> {
        Ok(self.is_inverted)
    }
}

impl HoverBoardMotor {
    pub fn new(pwm: Pwm, direction: Pin) -> Result<Self, LogData> {
        if pwm.set_duty_cycle_ns(0).is_err() {
            Err(LogData::fatal("Failed to set initial duty cycle!"))
        }else if pwm.set_period_ns(PERIOD_NS).is_err() {
            Err(LogData::fatal("Failed to set initial period!"))
        }else if pwm.enable(true).is_err() {
            Err(LogData::fatal("Failed to set enable motor controller!"))
        } else {
            Ok(HoverBoardMotor {
                is_inverted: false,
                pwm,
                direction,
            })
        }
    }
}

impl Drop for HoverBoardMotor {
    fn drop(&mut self) {
        self.stop();
    }
}