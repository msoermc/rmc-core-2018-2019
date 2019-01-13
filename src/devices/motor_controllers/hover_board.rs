use sysfs_gpio::Direction;
use sysfs_gpio::Pin;
use sysfs_pwm::Pwm;

use crate::devices::motor_controllers::MotorFailure;
use crate::devices::motor_controllers::MotorFailureKind;
use crate::logging::log_data::LogData;
use crate::robot_map::MotorID;

use super::MotorController;

const PERIOD_NS: u32 = 20_000;

pub struct HoverBoardMotor {
    is_inverted: bool,
    id: MotorID,
    pwm: Pwm,
    direction: Pin,
}

impl MotorController for HoverBoardMotor {
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
            Err(MotorFailure::new(self.id, MotorFailureKind::Unknown, LogData::error("Failed to set speed of hoverboard motor")))
        } else if set_direction().is_err() {
            Err(MotorFailure::new(self.id, MotorFailureKind::Unknown, LogData::error("Failed to set direction of hoverboard motor")))
        } else {
            Ok(())
        }
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        let set_duty = || {
            self.pwm.set_duty_cycle_ns(0)
        };

        if self.pwm.with_exported(set_duty).is_err() {
            Err(MotorFailure::new(self.id, MotorFailureKind::Unknown, LogData::error("Failed to stop hoverboard motor")))
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

impl HoverBoardMotor {
    pub fn create(pwm: Pwm, direction: Pin, id: MotorID) -> Result<Self, LogData> {
        if pwm.export().is_err() {
            Err(LogData::fatal("Failed to export pwm!"))
        } else if direction.export().is_err() {
            Err(LogData::fatal("Failed to export pin!"))
        } else if pwm.set_duty_cycle_ns(0).is_err() {
            Err(LogData::fatal("Failed to set the initial duty cycle!"))
        } else if pwm.set_period_ns(PERIOD_NS).is_err() {
            Err(LogData::fatal("Failed to set the initial period!"))
        } else if pwm.enable(true).is_err() {
            Err(LogData::fatal("Failed to enable initially!"))
        } else if direction.set_direction(Direction::Out).is_err() {
            Err(LogData::fatal("Failed to set initial direction!"))
        } else if direction.set_value(0).is_err() {
            Err(LogData::fatal("Failed to set initial pin value!"))
        } else {
            Ok(HoverBoardMotor {
                is_inverted: false,
                id,
                pwm,
                direction,
            })
        }
    }
}

impl Drop for HoverBoardMotor {
    fn drop(&mut self) {
        self.stop().expect("Failed to stop while dropping motor!");
    }
}