use crate::devices::motor_controllers::MotorController;

pub struct MotorGroup {
    is_inverted: bool,
    motors: Vec<Box<MotorController>>,
}

impl MotorController for MotorGroup {
    fn set_speed(&mut self, new_speed: f32) {
        for motor in &mut self.motors {
            motor.set_speed(new_speed);
        }
    }

    fn stop(&mut self) {
        self.set_speed(0.0)
    }

    fn invert(&mut self) {
        self.is_inverted = !self.is_inverted();

        for motor in &mut self.motors {
            motor.invert();
        }
    }

    fn is_inverted(&self) -> bool {
        self.is_inverted
    }
}

impl MotorGroup {
    pub fn new(motors: Vec<Box<MotorController>>) -> MotorGroup {
        MotorGroup {
            is_inverted: false,
            motors,
        }
    }
}