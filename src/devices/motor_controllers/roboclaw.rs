use sysfs_pwm::Pwm;

use crate::devices::motor_controllers::MotorFailure;
use crate::devices::motor_controllers::MotorFailureKind;
use crate::robot_map::MotorID;

use super::MotorController;

const PERIOD_NS: u32 = 20_000_000;
const ZERO_NS:   u32 = 1_500_000;
const RANGE_NS:  u32 = 500_000;

pub struct RoboClaw {
    is_inverted: bool,
    id: MotorID,
    pwm: Pwm,
}

impl MotorController for RoboClaw {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure> {
        let set_duty = || {
            if self.pwm.set_duty_cycle_ns(pwm_out.abs() as u32).is_err() {
                let pwm_out = new_speed * RANGE_NS as f32 + ZERO_NS as f32;
                self.pwm.export()?;
                self.pwm.set_duty_cycle_ns(pwm_out.abs() as u32)
            } else {
                Ok(())
            }
        };
        
        if set_duty().is_err() {
            error!("Failed to set duty cycle!");
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

impl RoboClaw {
    pub fn create(pwm: Pwm, id: MotorID) -> Result<Self, ()> {
        if pwm.export().is_err() {
            error!("Failed to export pwm!");
            Err(())
        } else if pwm.enable(true).is_err() {
            error!("Failed to enable initially!");
            Err(())
        } else if pwm.set_period_ns(PERIOD_NS).is_err() {
            error!("Failed to set initial period!");
            Err(())
        } else if pwm.set_duty_cycle_ns(ZERO_NS).is_err() {
            error!("Failed to set initial duty cycle!");
            Err(())
        } else {
            Ok(RoboClaw {
                is_inverted: false,
                id,
                pwm,
            })
        }
    }
}

/// When the motor is dropped, stop it.
impl Drop for RoboClaw {
    fn drop(&mut self) {
        if self.stop().is_err() {
            error!("Failed to stop while dropping motor!")
        };
    }
}
