use std::sync::Arc;

use crate::pinouts::analog::output::TestPwm;

use super::*;
use atomic::{Atomic, Ordering};

struct TestEnvironment {
    output: Arc<Atomic<f32>>,
    period: Arc<Atomic<usize>>,
    width: Arc<Atomic<usize>>,
    motor: RoboClaw,
}

fn setup() -> TestEnvironment {
    let output: Arc<Atomic<f32>> = Arc::new(Atomic::new(0.0));
    let period: Arc<Atomic<usize>> = Arc::new(Atomic::new(0));
    let width: Arc<Atomic<usize>> = Arc::new(Atomic::new(0));

    let pwm = Box::new(TestPwm::pwm(
        output.clone(),
        width.clone(),
        period.clone()));

    let motor = RoboClaw::new(pwm, Arc::new(GlobalMotorState::new()));

    TestEnvironment {
        output,
        period,
        width,
        motor,
    }
}

#[test]
fn init() {
    let env = setup();

    assert_eq!(BRAKE as usize, env.width.load(Ordering::SeqCst));
    assert_eq!(PERIOD as usize, env.period.load(Ordering::SeqCst));
}

#[test]
fn forwards() {
    let mut env = setup();

    env.motor.set_speed(1.00);

    assert_eq!(FULL_FORWARD as usize, env.width.load(Ordering::SeqCst));
    assert_eq!(PERIOD as usize, env.period.load(Ordering::SeqCst));
}

#[test]
fn backwards() {
    let mut env = setup();

    env.motor.set_speed(-1.00);

    assert_eq!(FULL_BACKWARD as usize, env.width.load(Ordering::SeqCst));
    assert_eq!(PERIOD as usize, env.period.load(Ordering::SeqCst));
}

#[test]
fn brake() {
    let mut env = setup();

    env.motor.set_speed(1.00);

    env.motor.set_speed(0.0);

    assert_eq!(BRAKE as usize, env.width.load(Ordering::SeqCst));
    assert_eq!(PERIOD as usize, env.period.load(Ordering::SeqCst));
}
