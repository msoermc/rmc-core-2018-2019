use crate::motor_controllers::GlobalMotorState;
use crate::pinouts::analog::output::PwmOutput;

use super::MotorController;

const OUTPUT_VOLTAGE: f32 = 3.3;

pub struct RoboClaw {
    pwm: Box<PwmOutput>,
    state: GlobalMotorState,
}

impl MotorController for RoboClaw {
    fn set_speed(&mut self, new_speed: f32) {
        let value = (new_speed + 1.0) / OUTPUT_VOLTAGE;
        self.pwm.set_value(value);
        self.state.set_speed(new_speed);
        println!("Claw: {}", value);
    }

    fn stop(&mut self) {
        self.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl RoboClaw {
    pub fn new(pwm: Box<PwmOutput>) -> Self {
        let mut result = RoboClaw {
            pwm,
            state: GlobalMotorState::new(),
        };

        result.set_speed(1.0);
        result
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

    use atomic::Atomic;
    use atomic::Ordering;

    use crate::pinouts::analog::output::TestPwm;

    use super::*;

    fn output_to_voltage(val: f32) -> f32 {
        (val + 1.0) / OUTPUT_VOLTAGE
    }

    #[test]
    fn test_init() {
        let output = Arc::new(atomic::Atomic::new(0.0));
        let pwm = Box::new(TestPwm::new(output.clone()));

        let _motor = RoboClaw::new(pwm);

        assert_eq!(output_to_voltage(0.0), output.load(Ordering::SeqCst));
    }

    #[test]
    fn forwards() {
        let output = Arc::new(Atomic::new(0.0));
        let pwm = Box::new(TestPwm::new(output.clone()));

        let mut motor = RoboClaw::new(pwm);

        motor.set_speed(1.0);

        assert_eq!(2.0 / OUTPUT_VOLTAGE, output.load(Ordering::SeqCst));
        assert_eq!(1.0, motor.get_motor_state().get_speed())
    }

    #[test]
    fn backwards() {
        let output = Arc::new(Atomic::new(0.0));
        let pwm = Box::new(TestPwm::new(output.clone()));

        let mut motor = RoboClaw::new(pwm);

        motor.set_speed(-1.0);

        assert_eq!(0.0, output.load(Ordering::SeqCst));
        assert_eq!(-1.0, motor.get_motor_state().get_speed())
    }

    #[test]
    fn brake() {
        let output = Arc::new(Atomic::new(0.0));
        let pwm = Box::new(TestPwm::new(output.clone()));

        let mut motor = RoboClaw::new(pwm);

        motor.set_speed(1.0);

        motor.stop();

        assert_eq!(1.0 / OUTPUT_VOLTAGE, output.load(Ordering::SeqCst));
        assert_eq!(0.0, motor.get_motor_state().get_speed())
    }
}