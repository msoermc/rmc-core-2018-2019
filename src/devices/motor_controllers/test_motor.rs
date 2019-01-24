use std::sync::Arc;
use std::sync::RwLock;

use super::*;

pub struct TestMotor {
    inverted: Arc<RwLock<bool>>,
    speed: Arc<RwLock<f32>>,
}

impl MotorController for TestMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorState> {
        let new_speed = if *self.inverted.read().unwrap() {
            -new_speed
        } else {
            new_speed
        };
        *self.speed.write().unwrap() = new_speed;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorState> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) -> Result<(), MotorState> {
        let inverted = *self.inverted.read().unwrap();
        *self.inverted.write().unwrap() = !inverted;
        self.stop()
    }

    fn is_inverted(&self) -> Result<bool, MotorState> {
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