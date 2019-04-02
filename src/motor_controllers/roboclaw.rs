use std::sync::Arc;

use crate::motor_controllers::GlobalMotorState;
use crate::pinouts::analog::output::PwmOutput;

use super::MotorController;

//Multiply by this and add PWM neutral pulse length to convert speed to pulse width (ns)
const OUTPUT_CONVERSION: f32 = 500.0;
//Motor is driven to neutral/stopped when PWM outputs 1500ns pulse
const PWM_NEUTRAL: f32 = 1500.0;

pub struct RoboClaw {
    pwm: Box<PwmOutput>,
    state: Arc<GlobalMotorState>,
}

impl MotorController for RoboClaw {
    fn set_speed(&mut self, new_speed: f32) {
        let value = (new_speed * OUTPUT_CONVERSION) + PWM_NEUTRAL;
        self.pwm.set_value(value);
        self.state.set_speed(new_speed);
    }

    fn stop(&mut self) {
        self.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl RoboClaw {
    pub fn new(pwm: Box<PwmOutput>, state: Arc<GlobalMotorState>) -> Self {
        let mut result = RoboClaw {
            pwm,
            state,
        };

        result.set_speed(0.0);
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
        (val * OUTPUT_CONVERSION) + PWM_NEUTRAL
    }

    #[test]
    fn test_init() {
        let output = Arc::new(atomic::Atomic::new(0.0));
        let pwm = Box::new(TestPwm::new(output.clone()));

        let _motor = RoboClaw::new(pwm, Arc::new(GlobalMotorState::new()));

        assert_eq!((0.0 * OUTPUT_CONVERSION) + PWM_NEUTRAL, output.load(Ordering::SeqCst));
    }

    #[test]
    fn forwards() {
        let output = Arc::new(Atomic::new(0.0));
        let pwm = Box::new(TestPwm::new(output.clone()));

        let mut motor = RoboClaw::new(pwm, Arc::new(GlobalMotorState::new()));

        motor.set_speed(1.0);

        assert_eq!((2.0 * OUTPUT_CONVERSION) + PWM_NEUTRAL, output.load(Ordering::SeqCst));
        assert_eq!((1.0 * OUTPUT_CONVERSION) + PWM_NEUTRAL, motor.get_motor_state().get_speed())
    }

    #[test]
    fn backwards() {
        let output = Arc::new(Atomic::new(0.0));
        let pwm = Box::new(TestPwm::new(output.clone()));

        let mut motor = RoboClaw::new(pwm, Arc::new(GlobalMotorState::new()));

        motor.set_speed(-1.0);

        assert_eq!((0.0 * OUTPUT_CONVERSION) + PWM_NEUTRAL, output.load(Ordering::SeqCst));
        assert_eq!((-1.0 * OUTPUT_CONVERSION) + PWM_NEUTRAL, motor.get_motor_state().get_speed())
    }

    #[test]
    fn brake() {
        let output = Arc::new(Atomic::new(0.0));
        let pwm = Box::new(TestPwm::new(output.clone()));

        let mut motor = RoboClaw::new(pwm, Arc::new(GlobalMotorState::new()));

        motor.set_speed(1.0);

        motor.stop();

        assert_eq!((1.0  * OUTPUT_CONVERSION) + PWM_NEUTRAL, output.load(Ordering::SeqCst));
        assert_eq!((0.0 * OUTPUT_CONVERSION) + PWM_NEUTRAL, motor.get_motor_state().get_speed())
    }
}