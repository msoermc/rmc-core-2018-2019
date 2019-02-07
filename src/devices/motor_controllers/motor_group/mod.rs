use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::GlobalMotorState;
use std::sync::Arc;

pub struct MotorGroup {
    motors: Vec<Box<MotorController>>,
    state: Arc<GlobalMotorState>,
}

impl MotorController for MotorGroup {
    fn set_speed(&mut self, new_speed: f32) {
        for motor in &mut self.motors {
            motor.set_speed(new_speed);
        }

        self.state.set_speed(new_speed);
    }

    fn stop(&mut self) {
        for motor in &mut self.motors {
            motor.stop();
        }

        self.state.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        unimplemented!()
    }
}

impl MotorGroup {
    pub fn new(motors: Vec<Box<MotorController>>, state: Arc<GlobalMotorState>) -> Self {
        Self {
            motors,
            state,
        }
    }
}