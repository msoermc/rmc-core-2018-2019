use std::sync::Arc;

use crate::motor_controllers::GlobalMotorState;
use crate::pinouts::analog::output::AnalogOutput;
use crate::pinouts::analog::output::PwmOutput;
use crate::pinouts::digital::output::DigitalOutput;

use super::MotorController;

pub struct RoboClaw {
    pwm: Box<PwmOutput>,
    state: GlobalMotorState,
}

impl MotorController for RoboClaw {
    fn set_speed(&mut self, new_speed: f32) {
        unimplemented!()
    }

    fn stop(&mut self) {
        self.pwm.set_value(0.0);
        self.state.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl RoboClaw {
    pub fn new(pwm: Box<PwmOutput>) -> Self {
        RoboClaw {
            pwm,
            state: GlobalMotorState::new(),
        }
    }
}

/// When the motor is dropped, stop it.
impl Drop for RoboClaw {
    fn drop(&mut self) {
        self.stop();
    }
}
