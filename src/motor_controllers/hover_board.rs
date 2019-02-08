use crate::motor_controllers::GlobalMotorState;
use crate::pinouts::AnalogOutput;
use crate::pinouts::DigitalOutput;

use super::MotorController;

pub struct HoverBoardMotor {
    pwm: Box<AnalogOutput>,
    direction: Box<DigitalOutput>,
    state: GlobalMotorState,
}

impl MotorController for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) {
        let speed = new_speed.abs();
        self.pwm.set_value(speed);
        let is_reverse = new_speed < 0.0;
        self.direction.set_value(is_reverse);
        self.state.set_speed(new_speed);
    }

    fn stop(&mut self) {
        self.pwm.set_value(0.0);
        self.state.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl HoverBoardMotor {
    pub fn new(pwm: Box<AnalogOutput>, direction: Box<DigitalOutput>) -> Self {
        HoverBoardMotor {
            pwm,
            direction,
            state: GlobalMotorState::new(),
        }
    }
}

/// When the motor is dropped, stop it.
impl Drop for HoverBoardMotor {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    use atomic::Atomic;
    use atomic::Ordering as ExtOrd;

    use crate::pinouts::TestPin;
    use crate::pinouts::TestPwm;

    use super::*;

    #[test]
    fn test_hobo() {
        let pwm_state = Arc::new(Atomic::new(0.0));
        let pin_state = Arc::new(AtomicBool::new(false));
        let pwm = Box::new(TestPwm::new(pwm_state.clone()));
        let pin = Box::new(TestPin::new(pin_state.clone()));

        let mut motor = HoverBoardMotor::new(pwm, pin);

        motor.set_speed(1.0);
        assert_eq!(1.0, motor.get_motor_state().get_speed());
        assert_eq!(false, pin_state.load(Ordering::Relaxed));
        assert_eq!(1.0, pwm_state.load(ExtOrd::Relaxed));

        motor.set_speed(-1.0);
        assert_eq!(-1.0, motor.get_motor_state().get_speed());
        assert_eq!(true, pin_state.load(Ordering::Relaxed));
        assert_eq!(1.0, pwm_state.load(ExtOrd::Relaxed));

        motor.stop();
        assert_eq!(0.0, motor.get_motor_state().get_speed());
        assert_eq!(0.0, pwm_state.load(ExtOrd::Relaxed));
    }
}