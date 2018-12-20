use super::*;

pub struct TestMotor {
    pub inverted: bool,
    pub speed: f32
}

impl MotorController for TestMotor {
    fn set_speed(&mut self, new_speed: f32) {
        self.speed = new_speed;
    }

    fn stop(&mut self) {
        self.set_speed(0.0);
    }

    fn invert(&mut self) {
        self.inverted = !self.inverted;
    }

    fn is_inverted(&self) -> bool {
        self.inverted
    }
}

impl TestMotor {
    pub fn new() -> TestMotor {
        TestMotor {
            inverted: false,
            speed: 0.0
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}