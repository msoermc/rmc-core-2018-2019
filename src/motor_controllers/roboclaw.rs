use crate::motor_controllers::GlobalMotorState;

use super::MotorController;
use crate::pinouts::analog::output::PwmOutput;
use crate::pinouts::analog::output::AnalogOutput;


pub struct RoboClaw {
    pwm: Box<PwmOutput>,
    state: GlobalMotorState,
}

impl MotorController for RoboClaw {
    fn set_speed(&mut self, new_speed: f32) {
//        self.pwm.set_pulse_width(new_speed);
//        let is_reverse = new_speed < 0.0;
//        self.state.set_speed(new_speed);
        unimplemented!()
    }
    
    fn stop(&mut self) {
        self.pwm.set_value(0.0);
        self.state.set_speed(0.0);
    }
    
    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl RoboClaw {
    pub fn new(pwm: Box<PwmOutput>) -> Self {
        RoboClaw {
            pwm,
            state: GlobalMotorState::new(),
        }
    }
}

/// When the motor is dropped, stop it.
impl Drop for RoboClaw {
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
    
    use super::*;
    
    #[test]
    fn test_roboclaw() {
//        let pwm_state = Arc::new(Atomic::new(0.0));
//        let pin_state = Arc::new(AtomicBool::new(false));
//        let pwm = Box::new(TestPwm::new(pwm_state.clone()));
//        let pin = Box::new(TestPin::new(pin_state.clone()));
//
//        let mut motor = RoboClaw::new(pwm, pin);
//
//        motor.set_speed(1.0);
//        assert_eq!(1.0, motor.get_motor_state().get_speed());
//        assert_eq!(false, pin_state.load(Ordering::Relaxed));
//        assert_eq!(1.0, pwm_state.load(ExtOrd::Relaxed));
//
//        motor.set_speed(-1.0);
//        assert_eq!(-1.0, motor.get_motor_state().get_speed());
//        assert_eq!(true, pin_state.load(Ordering::Relaxed));
//        assert_eq!(1.0, pwm_state.load(ExtOrd::Relaxed));
//
//        motor.stop();
//        assert_eq!(0.0, motor.get_motor_state().get_speed());
//        assert_eq!(0.0, pwm_state.load(ExtOrd::Relaxed));
    }
}
