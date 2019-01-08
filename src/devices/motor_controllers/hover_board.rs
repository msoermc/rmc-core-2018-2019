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
    /// Set the duty cycle and direction based on a speed value ranging from -1 to 1,
    /// where -1 is full speed reverse and 1 is full speed forward
    fn set_speed(&mut self, new_speed: f32) {
        self.pwm.with_exported(|| {
            self.pwm.enable(true).unwrap();
            self.pwm.set_period_ns(PERIOD_NS).unwrap();
            self.pwm.set_duty_cycle_ns((new_speed.abs() * PERIOD_NS as f32) as u32)
        }).unwrap();
        if (new_speed < 0.0 && !self.is_inverted) || new_speed > 0.0 && self.is_inverted {
            self.invert()
        }
    }
    
    /// Set duty cycle to zero to stop the motor
    fn stop(&mut self) {
        self.pwm.with_exported(|| {
            self.pwm.enable(true).unwrap();
            self.pwm.set_period_ns(PERIOD_NS).unwrap();
            self.pwm.set_duty_cycle_ns(0)
        }).unwrap();
    }
    
    /// Invert wheel driving direction
    fn invert(&mut self) {
        self.is_inverted = !self.is_inverted;
        self.direction.set_active_low(self.is_inverted);
    }
    
    /// Return true if reverse
    fn is_inverted(&self) -> bool {
        self.is_inverted
    }
}

impl HoverBoardMotor {
    pub fn new(pwm: Pwm, direction: Pin) -> Self {
        HoverBoardMotor {
            is_inverted: false,
            pwm,
            direction,
        }
    }
}
