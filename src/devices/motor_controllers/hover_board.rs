use super::MotorController;
use sysfs_pwm::{Pwm};
use sysfs_gpio::Pin;
use crate::logging::log_data::LogData;
use crate::devices::motor_controllers::MotorFailure;

const PERIOD_NS: u32 = 20_000;

pub struct HoverBoardMotor {
    is_inverted: bool,
    pwm: Pwm,
    direction: Pin,
}

impl MotorController for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure> {
        unimplemented!()
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        unimplemented!()
    }

    fn invert(&mut self) -> Result<(), MotorFailure> {
        unimplemented!()
    }

    fn is_inverted(&self) -> Result<bool, MotorFailure> {
        unimplemented!()
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