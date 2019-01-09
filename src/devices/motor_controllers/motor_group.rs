use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::MotorFailure;

pub struct MotorGroupError {
    failures: Vec<MotorFailure>,
}

pub struct MotorGroup {
    is_inverted: bool,
    motors: Vec<Box<MotorController>>,
}

impl MotorGroup {
    pub fn new(motors: Vec<Box<MotorController>>) -> Self {
        MotorGroup {
            is_inverted: false,
            motors,
        }
    }

    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorGroupError> {
        unimplemented!()
    }

    fn stop(&mut self) -> Result<(), MotorGroupError> {
        unimplemented!()
    }

    fn invert(&mut self) -> Result<(), MotorGroupError> {
        unimplemented!()
    }

    fn is_inverted(&self) -> Result<bool, MotorGroupError> {
        unimplemented!()
    }
}