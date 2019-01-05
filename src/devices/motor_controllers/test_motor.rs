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
    fn set_speed(&mut self, new_speed: f32) {
        self.speed = new_speed;
        self.test_channel.send(TestAction::SetSpeed(new_speed)).unwrap();
    }

    fn stop(&mut self) {
        self.speed = 0.0;
        self.test_channel.send(TestAction::Stop).unwrap();
    }

    fn invert(&mut self) {
        self.inverted = !self.inverted;
        self.test_channel.send(TestAction::Invert).unwrap();
    }

    fn is_inverted(&self) -> bool {
        self.inverted
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