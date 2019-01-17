use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::MotorFailure;

pub struct MotorGroup {
    is_inverted: bool,
    motors: Vec<Box<MotorController>>,
    old_speed: f32,
}

impl MotorGroup {
    pub fn new(motors: Vec<Box<MotorController>>) -> Self {
        MotorGroup {
            is_inverted: false,
            motors,
            old_speed: 0.0,
        }
    }

    pub fn set_speed(&mut self, new_speed: f32) -> Result<(), Vec<MotorFailure>> {
        self.old_speed = new_speed;
        self.run_operation(|motor| motor.set_speed(new_speed))
    }

    pub fn stop(&mut self) -> Result<(), Vec<MotorFailure>> {
        self.old_speed = 0.0;
        self.run_operation(|motor| motor.stop())
    }

    pub fn invert(&mut self) -> Result<(), Vec<MotorFailure>> {
        self.is_inverted = !self.is_inverted;
        self.old_speed = -self.old_speed;
        self.run_operation(|motor| motor.invert())
    }

    pub fn is_inverted(&self) -> bool {
        self.is_inverted
    }

    pub fn maintain_last(&mut self) -> Result<(), Vec<MotorFailure>> {
        self.set_speed(self.old_speed)
    }

    fn run_operation<T: Fn(&mut Box<MotorController>) -> Result<(), MotorFailure>>(&mut self, operation: T) -> Result<(), Vec<MotorFailure>> {
        let results: Vec<MotorFailure> =
            self.motors.iter_mut()
                .map(operation)
                .filter_map(|res| res.err())
                .collect();

        if results.is_empty() {
            Ok(())
        } else {
            Err(results)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::RwLock;
    use crate::devices::motor_controllers::test_motor::TestMotor;

    #[test]
    fn test_set_speed() {
        let speed_0 = Arc::new(RwLock::new(0.0));
        let inverted_0 = Arc::new(RwLock::new(false));
        let motor_0 = TestMotor::new(speed_0.clone(), inverted_0.clone());

        let speed_1 = Arc::new(RwLock::new(0.0));
        let inverted_1 = Arc::new(RwLock::new(false));
        let motor_1 = TestMotor::new(speed_1.clone(), inverted_1.clone());

        let mut group = MotorGroup::new(vec![Box::new(motor_0), Box::new(motor_1)]);

        group.set_speed(1.0).unwrap();

        assert_eq!(1.0, *speed_0.read().unwrap());
        assert_eq!(1.0, *speed_1.read().unwrap());

        assert_eq!(false, *inverted_0.read().unwrap());
        assert_eq!(false, *inverted_1.read().unwrap());
    }
}