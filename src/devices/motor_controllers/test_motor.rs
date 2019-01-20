use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::RwLock;

use super::*;

pub struct TestMotor {
    inverted: Arc<RwLock<bool>>,
    speed: Arc<RwLock<f32>>,
}

impl MotorController for TestMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure> {
        let new_speed = if *self.inverted.read().unwrap() { -new_speed } else { new_speed };
        *self.speed.write().unwrap() = new_speed;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) -> Result<(), MotorFailure> {
        let mut inverted = *self.inverted.write().unwrap();
        inverted = !inverted;
        self.stop()
    }

    fn is_inverted(&self) -> Result<bool, MotorFailure> {
        Ok(*self.inverted.read().unwrap())
    }
}

impl TestMotor {
    pub fn new(inverted: Arc<RwLock<bool>>, speed: Arc<RwLock<f32>>) -> TestMotor {
        TestMotor {
            inverted,
            speed,
        }
    }
}