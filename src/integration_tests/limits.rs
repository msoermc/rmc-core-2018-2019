use std::sync::atomic::AtomicBool;

use atomic::Ordering;

use crate::pinouts::digital::TestPin;

use super::*;

const TIMEOUT_MILLIS: u64 = 50;

#[test]
fn upper_digger_limit() {
    let left_input = Arc::new(AtomicBool::new(false));
    let right_input = Arc::new(AtomicBool::new(false));

    let left_pin = Box::new(TestPin::new(left_input.clone()));
    let right_pin = Box::new(TestPin::new(right_input.clone()));

    let mut builder = RobotAssemblyBuilder::new();

    builder.with_test();

    builder.with_test_upper_left_intake_limit(left_pin);
    builder.with_test_upper_right_intake_limit(right_pin);

    let state = builder.get_state();

    let actuator = state.get_intake().get_actuator();

    let client = builder.generate().assemble().launch().engage_testing_server();

    let left_limit = state.get_intake().get_left_actuator().get_upper();
    let right_limit = state.get_intake().get_right_actuator().get_upper();

    client.post("/robot/modes/dig").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post("/robot/intake/rails/raise").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(MH_ACTUATOR_RATE, state.get_intake().get_actuator().get_speed());
    assert_eq!(false, left_limit.load(Ordering::SeqCst));
    assert_eq!(false, right_limit.load(Ordering::SeqCst));

    left_input.store(true, Ordering::SeqCst);

    client.post("/robot/intake/rails/raise").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(true, left_limit.load(Ordering::SeqCst));
    assert_eq!(false, right_limit.load(Ordering::SeqCst));

    left_input.store(false, Ordering::SeqCst);
    right_input.store(true, Ordering::SeqCst);

    client.post("/robot/intake/rails/raise").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(false, left_limit.load(Ordering::SeqCst));
    assert_eq!(true, right_limit.load(Ordering::SeqCst));

    left_input.store(true, Ordering::SeqCst);
    right_input.store(true, Ordering::SeqCst);

    client.post("/robot/intake/rails/raise").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(true, left_limit.load(Ordering::SeqCst));
    assert_eq!(true, right_limit.load(Ordering::SeqCst));
}

#[test]
fn lower_digger_limit() {
    let left_input = Arc::new(AtomicBool::new(false));
    let right_input = Arc::new(AtomicBool::new(false));

    let left_pin = Box::new(TestPin::new(left_input.clone()));
    let right_pin = Box::new(TestPin::new(right_input.clone()));

    let mut builder = RobotAssemblyBuilder::new();

    builder.with_test();

    builder.with_test_lower_left_intake_limit(left_pin);
    builder.with_test_lower_right_intake_limit(right_pin);

    let state = builder.get_state();

    let actuator = state.get_intake().get_actuator();

    let client = builder.generate().assemble().launch().engage_testing_server();

    let left_limit = state.get_intake().get_left_actuator().get_lower();
    let right_limit = state.get_intake().get_right_actuator().get_lower();

    client.post("/robot/modes/dig").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post("/robot/intake/rails/lower").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(-MH_ACTUATOR_RATE, state.get_intake().get_actuator().get_speed());
    assert_eq!(false, left_limit.load(Ordering::SeqCst));
    assert_eq!(false, right_limit.load(Ordering::SeqCst));

    left_input.store(true, Ordering::SeqCst);

    client.post("/robot/intake/rails/lower").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(true, left_limit.load(Ordering::SeqCst));
    assert_eq!(false, right_limit.load(Ordering::SeqCst));

    left_input.store(false, Ordering::SeqCst);
    right_input.store(true, Ordering::SeqCst);

    client.post("/robot/intake/rails/lower").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(false, left_limit.load(Ordering::SeqCst));
    assert_eq!(true, right_limit.load(Ordering::SeqCst));

    left_input.store(true, Ordering::SeqCst);
    right_input.store(true, Ordering::SeqCst);

    client.post("/robot/intake/rails/lower").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(true, left_limit.load(Ordering::SeqCst));
    assert_eq!(true, right_limit.load(Ordering::SeqCst));
}