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
        if let Err(_) = self.pwm.with_exported(|| {
            let pwm_out = new_speed * PERIOD_NS as f32;
            self.pwm.set_duty_cycle_ns(pwm_out as u32)
        })
        {
            Err(MotorFailure::new(self.id, MotorFailureKind::Unknown, LogData::error("Failed to set speed of hoverboard motor")))
        } else if let Err(_) = self.direction.with_exported(|| {
            let is_reverse = new_speed < 0.0;
            self.direction.set_active_low(is_reverse ^ self.is_inverted)
        })
        {
            Err(MotorFailure::new(self.id, MotorFailureKind::Unknown, LogData::error("Failed to set description of hoverboard motor")))
        } else {
            Ok(())
        }
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        if let Err(_) = self.pwm.with_exported(|| {
            self.pwm.set_duty_cycle_ns(0)
        })
        {
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
    pub fn new(pwm: Pwm, direction: Pin, id: MotorID) -> Result<Self, LogData> {
        if pwm.set_duty_cycle_ns(0).is_err() {
            Err(LogData::fatal("Failed to set initial duty cycle!"))
        } else if pwm.set_period_ns(PERIOD_NS).is_err() {
            Err(LogData::fatal("Failed to set initial period!"))
        } else if pwm.enable(true).is_err() {
            Err(LogData::fatal("Failed to set enable motor controller!"))
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