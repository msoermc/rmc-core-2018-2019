use sysfs_gpio::Direction;
use sysfs_gpio::Pin;
use sysfs_pwm::Pwm;

use crate::devices::motor_controllers::MotorFailure;
use crate::devices::motor_controllers::MotorFailureKind;
use crate::robot_map::MotorID;

use super::MotorController;

const PERIOD_NS: u32 = 20_000;

pub struct PwmMotor {
    is_inverted: bool,
    id: MotorID,
    pwm: Pwm,
    direction: Pin,
}

impl MotorController for PwmMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure> {
        let set_duty = || {
            let pwm_out = new_speed * PERIOD_NS as f32;
            if self.pwm.set_duty_cycle_ns(pwm_out.abs() as u32).is_err() {
                self.pwm.export()?;
                self.pwm.set_duty_cycle_ns(pwm_out.abs() as u32)
            } else {
                Ok(())
            }
        };

        let set_direction = || {
            let is_reverse = new_speed < 0.0;
            self.direction.set_value((is_reverse ^ self.is_inverted) as u8)
        };

        if set_duty().is_err() {
            error!("Failed to set duty cycle!");
            Err(MotorFailure::new(self.id, MotorFailureKind::Unknown))
        } else if set_direction().is_err() {
            error!("Failed to set motor direction!");
            Err(MotorFailure::new(self.id, MotorFailureKind::Unknown))
        } else {
            Ok(())
        }
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        let set_duty = || {
            self.pwm.set_duty_cycle_ns(0)
        };

        if self.pwm.with_exported(set_duty).is_err() {
            error!("Failed to stop motor!");
            Err(MotorFailure::new(self.id, MotorFailureKind::Unknown))
        } else {
            Ok(())
        }
    }

    fn invert(&mut self) -> Result<(), MotorFailure> {
        self.is_inverted = !self.is_inverted;

        Ok(())
    }

    fn is_inverted(&self) -> Result<bool, MotorFailure> {
        Ok(self.is_inverted)
    }
}

impl PwmMotor {
    pub fn create(pwm: Pwm, direction: Pin, id: MotorID) -> Result<Self, ()> {
        if pwm.export().is_err() {
            error!("Failed to export pwm!");
            Err(())
        } else if direction.export().is_err() {
            error!("Failed to export pin!");
            Err(())
        } else if pwm.enable(true).is_err() {
            error!("Failed to enable initially!");
            Err(())
        } else if pwm.set_period_ns(PERIOD_NS).is_err() {
            error!("Failed to set initial period!");
            Err(())
        } else if pwm.set_duty_cycle_ns(0).is_err() {
            error!("Failed to set initial duty cycle!");
            Err(())
        } else if direction.set_direction(Direction::Out).is_err() {
            error!("Failed to set initial direction!");
            Err(())
        } else if direction.set_value(0).is_err() {
            error!("Failed to set initial pin value!");
            Err(())
        } else {
            Ok(PwmMotor {
                is_inverted: false,
                id,
                pwm,
                direction,
            })
        }
    }
}

/// When the motor is dropped, stop it.
impl Drop for PwmMotor {
    fn drop(&mut self) {
        if self.stop().is_err() {
            error!("Failed to stop while dropping motor!")
        };
    }
}