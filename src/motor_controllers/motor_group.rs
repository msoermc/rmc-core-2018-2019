use std::sync::Arc;

use crate::motor_controllers::GlobalMotorState;
use crate::motor_controllers::MotorController;

pub struct MotorGroup {
    motors: Vec<Box<dyn MotorController>>,
    state: Arc<GlobalMotorState>,
}

impl MotorController for MotorGroup {
    fn set_speed(&mut self, new_speed: f32) {
        for motor in &mut self.motors {
            motor.set_speed(new_speed);
        }

        self.state.set_speed(new_speed);
    }

    fn stop(&mut self) {
        for motor in &mut self.motors {
            motor.stop();
        }

        self.state.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl MotorGroup {
    pub fn new(motors: Vec<Box<dyn MotorController>>, state: Arc<GlobalMotorState>) -> Self {
        Self {
            motors,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::motor_controllers::test_motor::TestMotor;

    use super::*;

    #[test]
    fn test_motor_group() {
        let state = Arc::new(GlobalMotorState::new());
        let test_state = Arc::new(GlobalMotorState::new());
        let motor = Box::new(TestMotor::new(test_state));
        let mut motor = MotorGroup::new(vec![motor], state.clone());
        assert_eq!(0.0, motor.get_motor_state().get_speed());

        motor.set_speed(1.0);
        assert_eq!(1.0, motor.get_motor_state().get_speed());
        assert_eq!(1.0, motor.get_motor_state().get_current_state().get_speed());

        motor.set_speed(-1.0);
        assert_eq!(-1.0, motor.get_motor_state().get_speed());
        assert_eq!(-1.0, motor.get_motor_state().get_current_state().get_speed());

        motor.stop();
        assert_eq!(0.0, motor.get_motor_state().get_speed());
        assert_eq!(0.0, motor.get_motor_state().get_current_state().get_speed());

        motor.set_speed(1.0);
        assert_eq!(1.0, motor.get_motor_state().get_speed());
        assert_eq!(1.0, motor.get_motor_state().get_current_state().get_speed());
    }
}