use std::sync::Arc;

use crate::motor_controllers::GlobalMotorState;
use crate::pinouts::analog::output::PwmOutput;

use super::MotorController;

#[cfg(test)]
mod tests;

// 1.50 ms = 1,500 us = 1,500,000 ns
const BRAKE: u32 = 1_500_000;

// 1.75 ms = 1,750 us = 1,750,000 ns
const FULL_FORWARD: u32 = 1_750_000;

// 1.25 ms = 1,250 us = 1,250,000 ns
const FULL_BACKWARD: u32 = 1_250_000;

// 2.00 ms = 2,000 us = 2,000,000 ns
const PERIOD: u32 = 20_000_000;

//Multiply by this and add PWM neutral pulse length to convert speed to pulse width (ns)
const COEFFICIENT: f32 = (FULL_FORWARD - BRAKE) as f32;

pub struct RoboClaw {
    pwm: Box<PwmOutput>,
    state: Arc<GlobalMotorState>,
}

impl MotorController for RoboClaw {
    fn set_speed(&mut self, new_speed: f32) {
        let base_speed = (new_speed * COEFFICIENT) as i32;
        let width = (base_speed + BRAKE as i32) as u32;
        self.pwm.set_pulse_duty_cycle(width);
        self.state.set_speed(new_speed);
        info!("Speed: {}", width);
    }

    fn stop(&mut self) {
        self.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl RoboClaw {
    pub fn new(mut pwm: Box<PwmOutput>, state: Arc<GlobalMotorState>) -> Self {
        pwm.set_period(PERIOD);
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