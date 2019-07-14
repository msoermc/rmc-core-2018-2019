use crate::motor_controllers::GlobalMotorState;
use crate::motor_controllers::MotorController;

pub struct InvertedMotor {
    motor: Box<dyn MotorController>,
}

impl MotorController for InvertedMotor {
    fn set_speed(&mut self, new_speed: f32) {
        self.motor.set_speed(-new_speed);
    }

    fn stop(&mut self) {
        self.motor.stop()
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        self.motor.get_motor_state()
    }
}

impl InvertedMotor {
    pub fn new(motor: Box<dyn MotorController>) -> Self {
        Self {
            motor,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::motor_controllers::test_motor::TestMotor;

    use super::*;

    #[test]
    fn test_inversion() {
        let state = Arc::new(GlobalMotorState::new());
        let motor = Box::new(TestMotor::new(state.clone()));
        let mut motor = InvertedMotor::new(motor);

        assert_eq!(0.0, motor.get_motor_state().get_speed());

        motor.set_speed(1.0);
        assert_eq!(-1.0, motor.get_motor_state().get_speed());

        motor.set_speed(-1.0);
        assert_eq!(1.0, motor.get_motor_state().get_speed());

        motor.stop();
        assert_eq!(0.0, motor.get_motor_state().get_speed());
    }
}