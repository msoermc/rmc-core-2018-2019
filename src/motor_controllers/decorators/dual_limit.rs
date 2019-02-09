use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use atomic::Ordering;

use crate::motor_controllers::GlobalMotorState;
use crate::motor_controllers::MotorController;

pub struct DualLimitMotor {
    motor: Box<MotorController>,
    upper: Arc<AtomicBool>,
    lower: Arc<AtomicBool>,
}

impl MotorController for DualLimitMotor {
    fn set_speed(&mut self, new_speed: f32) {
        if (new_speed > 0.0 && !self.upper.load(Ordering::SeqCst))
            || (new_speed < 0.0 && !self.lower.load(Ordering::SeqCst)) {
            self.motor.set_speed(new_speed);
        } else {
            self.stop();
        }
    }

    fn stop(&mut self) {
        self.motor.stop();
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        self.motor.get_motor_state()
    }
}

impl DualLimitMotor {
    pub fn new(motor: Box<MotorController>, upper: Arc<AtomicBool>, lower: Arc<AtomicBool>) -> Self {
        Self {
            motor,
            upper,
            lower,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use crate::motor_controllers::test_motor::TestMotor;

    use super::*;

    #[test]
    fn test_raise() {
        let state = Arc::new(GlobalMotorState::new());
        let upper = Arc::new(AtomicBool::new(false));
        let lower = Arc::new(AtomicBool::new(false));
        let motor = Box::new(TestMotor::new(state.clone()));
        let mut motor = DualLimitMotor::new(motor, upper.clone(), lower.clone());

        assert_eq!(0.0, motor.get_motor_state().get_speed());

        motor.set_speed(1.0);
        assert_eq!(1.0, motor.get_motor_state().get_speed());

        motor.stop();
        assert_eq!(0.0, motor.get_motor_state().get_speed());

        upper.store(true, Ordering::Relaxed);
        motor.set_speed(1.0);
        assert_eq!(0.0, motor.get_motor_state().get_speed());
    }

    #[test]
    fn test_lower() {
        let state = Arc::new(GlobalMotorState::new());
        let upper = Arc::new(AtomicBool::new(false));
        let lower = Arc::new(AtomicBool::new(false));
        let motor = Box::new(TestMotor::new(state.clone()));
        let mut motor = DualLimitMotor::new(motor, upper.clone(), lower.clone());

        assert_eq!(0.0, motor.get_motor_state().get_speed());

        motor.set_speed(-1.0);
        assert_eq!(-1.0, motor.get_motor_state().get_speed());

        motor.stop();
        assert_eq!(0.0, motor.get_motor_state().get_speed());

        lower.store(true, Ordering::Relaxed);
        motor.set_speed(-1.0);
        assert_eq!(0.0, motor.get_motor_state().get_speed());
    }
}