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
        for motor in &mut self.motors {
            motor.stop();
        }
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

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use std::sync::mpsc::Receiver;
    use std::time::Duration;

    use crate::devices::motor_controllers::test_motor::TestAction;
    use crate::devices::motor_controllers::test_motor::TestMotor;

    use super::*;

    const TIMEOUT_MS: u64 = 50;

    fn get_test_motor() -> (TestMotor, Receiver<TestAction>) {
        let (test_sender, test_receiver) = channel();
        let test_motor = TestMotor::new(test_sender);

        (test_motor, test_receiver)
    }

    #[test]
    fn test_set_speeds_single_motor() {
        let (motor, receiver) = get_test_motor();

        let mut motor_group = MotorGroup::new(vec![Box::new(motor)]);
        let mut receiver_group = vec![receiver];

        assert_set_speed(&mut motor_group, &mut receiver_group, 1.0);
        assert_set_speed(&mut motor_group, &mut receiver_group, -1.0);
        assert_set_speed(&mut motor_group, &mut receiver_group, 0.5);
        assert_set_speed(&mut motor_group, &mut receiver_group, -0.5);
        assert_set_speed(&mut motor_group, &mut receiver_group, 0.0);
    }

    #[test]
    fn test_set_speeds_two_motors() {
        let (motor_1, receiver_1) = get_test_motor();
        let (motor_2, receiver_2) = get_test_motor();

        let mut motor_group = MotorGroup::new(vec![Box::new(motor_1), Box::new(motor_2)]);
        let mut receiver_group = vec![receiver_1, receiver_2];

        assert_set_speed(&mut motor_group, &mut receiver_group, 1.0);
        assert_set_speed(&mut motor_group, &mut receiver_group, -1.0);
        assert_set_speed(&mut motor_group, &mut receiver_group, 0.5);
        assert_set_speed(&mut motor_group, &mut receiver_group, -0.5);
        assert_set_speed(&mut motor_group, &mut receiver_group, 0.0);
    }

    #[test]
    fn test_invert_single_motor() {
        let (motor, receiver) = get_test_motor();

        let mut motor_group = MotorGroup::new(vec![Box::new(motor)]);
        let mut receiver_group = vec![receiver];

        assert_inverted(&mut motor_group, &mut receiver_group);
        assert_inverted(&mut motor_group, &mut receiver_group);
    }

    #[test]
    fn test_invert_two_motors() {
        let (motor_1, receiver_1) = get_test_motor();
        let (motor_2, receiver_2) = get_test_motor();

        let mut motor_group = MotorGroup::new(vec![Box::new(motor_1), Box::new(motor_2)]);
        let mut receiver_group = vec![receiver_1, receiver_2];

        assert_inverted(&mut motor_group, &mut receiver_group);
        assert_inverted(&mut motor_group, &mut receiver_group);
    }

    #[test]
    fn test_stop_single_motor() {
        let (motor, receiver) = get_test_motor();

        let mut motor_group = MotorGroup::new(vec![Box::new(motor)]);
        let mut receiver_group = vec![receiver];

        assert_stop(&mut motor_group, &mut receiver_group);
    }

    #[test]
    fn test_stop_two_motors() {
        let (motor_1, receiver_1) = get_test_motor();
        let (motor_2, receiver_2) = get_test_motor();

        let mut motor_group = MotorGroup::new(vec![Box::new(motor_1), Box::new(motor_2)]);
        let mut receiver_group = vec![receiver_1, receiver_2];

        assert_stop(&mut motor_group, &mut receiver_group);
    }

    fn assert_stop(motor_group: &mut MotorGroup, receivers: &mut Vec<Receiver<TestAction>>) {
        motor_group.stop();

        for receiver in receivers {
            assert_eq!(TestAction::Stop,
                       receiver.recv_timeout(Duration::from_millis(TIMEOUT_MS)).unwrap());
        }
    }

    fn assert_set_speed(motor_group: &mut MotorGroup, receivers: &mut Vec<Receiver<TestAction>>, speed: f32) {
        motor_group.set_speed(speed);

        for receiver in receivers {
            assert_eq!(TestAction::SetSpeed(speed),
                       receiver.recv_timeout(Duration::from_millis(TIMEOUT_MS)).unwrap());
        }
    }

    fn assert_inverted(motor_group: &mut MotorGroup, receivers: &mut Vec<Receiver<TestAction>>) {
        motor_group.invert();

        for receiver in receivers {
            assert_eq!(TestAction::Invert,
                       receiver.recv_timeout(Duration::from_millis(TIMEOUT_MS)).unwrap());
        }
    }
}