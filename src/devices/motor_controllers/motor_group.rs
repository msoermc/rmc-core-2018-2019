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

    struct TestMotorGroup {
        pub inverted: Arc<RwLock<bool>>,
        pub speed: Arc<RwLock<f32>>,
        pub motor_group: MotorGroup,
    }

    fn create_group() -> TestMotorGroup {
        let inverted = Arc::new(RwLock::new(false));

        let speed = Arc::new(RwLock::new(0.0));

        let test_motor = TestMotor::new(inverted.clone(), speed.clone());

        let test_group = MotorGroup::new(vec![Box::new(test_motor)]);

        TestMotorGroup {inverted, speed, motor_group: test_group}
    }

    #[test]
    fn test_set_speed_no_fail_no_inversion() {
        let mut group = create_group();

        // Test setup
        assert_eq!(0.0, *group.speed.read().unwrap());
        assert_eq!(false, *group.inverted.read().unwrap());

        // Go forwards
        group.motor_group.set_speed(1.0).expect("Command should not have failed!");
        assert_eq!(1.0, *group.speed.read().unwrap());
        assert_eq!(false, *group.inverted.read().unwrap());

        // Go forwards
        group.motor_group.set_speed(-1.0).expect("Command should not have failed!");
        assert_eq!(-1.0, *group.speed.read().unwrap());
        assert_eq!(false, *group.inverted.read().unwrap());
    }
}