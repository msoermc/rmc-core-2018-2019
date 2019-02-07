use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::devices::motor_controllers::GlobalMotorState;
use crate::devices::motor_controllers::MotorController;

pub struct FlagMotor {
    motor: Box<MotorController>,
    disabled: Arc<AtomicBool>,
}

impl MotorController for FlagMotor {
    fn set_speed(&mut self, new_speed: f32) {
        if self.disabled.load(Ordering::SeqCst) {
            self.motor.stop();
        } else {
            self.motor.set_speed(new_speed);
        }
    }

    fn stop(&mut self) {
        self.motor.stop();
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        self.motor.get_motor_state()
    }
}

impl FlagMotor {
    pub fn new(motor: Box<MotorController>, disabled: Arc<AtomicBool>) -> Self {
        Self {
            motor,
            disabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use crate::devices::motor_controllers::test_motor::TestMotor;

    use super::*;

    #[test]
    fn test_flag() {
        let state = Arc::new(GlobalMotorState::new());
        let disabled = Arc::new(AtomicBool::new(false));
        let motor = Box::new(TestMotor::new(state.clone()));
        let mut motor = FlagMotor::new(motor, disabled.clone());

        assert_eq!(0.0, motor.get_motor_state().get_speed());

        motor.set_speed(1.0);
        assert_eq!(1.0, motor.get_motor_state().get_speed());

        motor.stop();
        assert_eq!(0.0, motor.get_motor_state().get_speed());

        disabled.store(true, Ordering::Relaxed);
        motor.set_speed(1.0);
        assert_eq!(0.0, motor.get_motor_state().get_speed());
    }
}