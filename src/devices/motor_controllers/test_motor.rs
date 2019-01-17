use std::sync::mpsc::Sender;

use super::*;

pub struct TestMotor {
    inverted: bool,
    speed: f32,
}

impl MotorController for TestMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure> {
        self.speed = new_speed;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) -> Result<(), MotorFailure> {
        self.inverted = !self.inverted;
        self.stop()
    }

    fn is_inverted(&self) -> Result<bool, MotorFailure> {
        Ok(self.inverted)
    }
}

impl TestMotor {
    pub fn new() -> TestMotor {
        TestMotor {
            inverted: false,
            speed: 0.0,
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}