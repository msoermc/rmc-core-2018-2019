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
