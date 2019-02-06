use std::sync::Arc;

use super::*;

pub struct TestMotor {
    state: Arc<GlobalMotorState>,
}

impl MotorController for TestMotor {
    fn set_speed(&mut self, new_speed: f32) {
        self.state.set_speed(new_speed);
    }

    fn stop(&mut self) {
        self.set_speed(0.0)
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl TestMotor {
    pub fn new(state: Arc<GlobalMotorState>) -> TestMotor {
        TestMotor {
            state
        }
    }
}