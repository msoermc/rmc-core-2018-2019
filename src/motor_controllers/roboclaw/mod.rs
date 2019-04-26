use std::sync::Arc;

use crate::motor_controllers::GlobalMotorState;
use crate::pinouts::analog::output::PwmOutput;

use super::MotorController;

#[cfg(test)]
mod tests;

//Multiply by this and add PWM neutral pulse length to convert speed to pulse width (ns)
const OUTPUT_CONVERSION: f32 = 0.25;
//Motor is driven to neutral/stopped when PWM outputs 1500ns pulse
const PWM_NEUTRAL: f32 = 0.75;

pub struct RoboClaw {
    pwm: Box<PwmOutput>,
    state: Arc<GlobalMotorState>,
}

impl MotorController for RoboClaw {
    fn set_speed(&mut self, new_speed: f32) {
        let value = (new_speed * OUTPUT_CONVERSION) + PWM_NEUTRAL;
        self.pwm.set_value(value);
        self.state.set_speed(new_speed);
    }

    fn stop(&mut self) {
        self.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl RoboClaw {
    pub fn new(pwm: Box<PwmOutput>, state: Arc<GlobalMotorState>) -> Self {
        let mut result = RoboClaw {
            pwm,
            state,
        };

        result.set_speed(0.0);
        result
    }
}

/// When the motor is dropped, stop it.
impl Drop for RoboClaw {
    fn drop(&mut self) {
        self.stop();
    }
}