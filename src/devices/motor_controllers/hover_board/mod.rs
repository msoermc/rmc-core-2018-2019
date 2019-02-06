use crate::devices::AnalogOutput;
use crate::devices::DigitalOutput;
use crate::devices::motor_controllers::GlobalMotorState;
use crate::robot_map::MotorID;

use super::MotorController;

#[cfg(test)]
mod tests;

pub struct HoverBoardMotor {
    is_inverted: bool,
    pwm: Box<AnalogOutput>,
    direction: Box<DigitalOutput>,
    state: GlobalMotorState,
}

impl MotorController for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) {
        self.pwm.set_value(new_speed);
        let is_reverse = new_speed < 0.0;
        self.direction.set_value(is_reverse ^ self.is_inverted);
    }

    fn stop(&mut self) {
        self.pwm.set_value(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl HoverBoardMotor {
    pub fn new(pwm: Box<AnalogOutput>, direction: Box<DigitalOutput>) -> Self {
        HoverBoardMotor {
            is_inverted: false,
            pwm,
            direction,
            state: GlobalMotorState::new(),
        }
    }
}


/// When the motor is dropped, stop it.
impl Drop for HoverBoardMotor {
    fn drop(&mut self) {
        self.stop();
    }
}