use super::*;
use std::sync::RwLock;
use std::sync::Arc;

pub struct TestMotor {
    inverted: Arc<RwLock<bool>>,
    speed: Arc<RwLock<f32>>,
}

impl MotorController for TestMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure> {
        *self.speed.write().unwrap() = new_speed;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) -> Result<(), MotorFailure> {
        let mut inv = *self.inverted.write().unwrap();
        inv = !inv;
        self.stop()
    }

    fn is_inverted(&self) -> Result<bool, MotorFailure> {
        Ok(*self.inverted.read().unwrap())
    }
}

impl TestMotor {
    pub fn new(speed: Arc<RwLock<f32>>, inverted: Arc<RwLock<bool>>) -> TestMotor {
        TestMotor {
            inverted,
            speed,
        }
    }

    pub fn get_speed(&self) -> f32 {
        *self.speed.read().unwrap()
    }
}