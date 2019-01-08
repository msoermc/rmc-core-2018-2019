use super::*;
use std::sync::mpsc::Sender;

pub struct TestMotor {
    inverted: bool,
    speed: f32,
    test_channel: Sender<TestAction>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TestAction {
    SetSpeed(f32),
    Stop,
    Invert,
}

impl MotorController for TestMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), LogData> {
        unimplemented!()
    }

    fn stop(&mut self) -> Result<(), LogData> {
        unimplemented!()
    }

    fn invert(&mut self) -> Result<(), LogData> {
        unimplemented!()
    }

    fn is_inverted(&self) -> Result<bool, LogData> {
        unimplemented!()
    }
}

impl TestMotor {
    pub fn new(test_channel: Sender<TestAction>) -> TestMotor {
        TestMotor {
            inverted: false,
            speed: 0.0,
            test_channel,
        }
    }
}