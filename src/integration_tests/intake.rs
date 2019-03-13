use std::sync::atomic::{AtomicBool, Ordering};

use rocket::http::ContentType;
use rocket::local::LocalResponse;

use crate::pinouts::digital::TestPin;

use super::*;

const TIMEOUT_MILLIS: u64 = 30;

fn enable_intake(client: &Client) -> LocalResponse {
    client.put("/robot")
        .header(ContentType::JSON)
        .body(r#"{ "mode" : "Digging" }"#)
        .dispatch()
}

fn send_dig(client: &Client) -> LocalResponse {
    client.put("/robot/intake")
        .header(ContentType::JSON)
        .body(r#"{ "digger" : "Dig" }"#)
        .dispatch()
}

fn send_stop_digging(client: &Client) -> LocalResponse {
    client.put("/robot/intake")
        .header(ContentType::JSON)
        .body(r#"{"digger":"Stop"}"#)
        .dispatch()
}

fn send_stop_actuators(client: &Client) -> LocalResponse {
    client.put("/robot/intake")
        .header(ContentType::JSON)
        .body(r#"{"actuator":"Stop"}"#)
        .dispatch()
}

fn send_raise(client: &Client) -> LocalResponse {
    let res = client.put("/robot/intake")
        .header(ContentType::JSON)
        .body(r#"{"actuator":"Raise"}"#)
        .dispatch();

    assert_eq!(Status::Ok, res.status());
    res
}

fn send_lower(client: &Client) -> LocalResponse {
    let res = client.put("/robot/intake")
        .header(ContentType::JSON)
        .body(r#"{"actuator":"Lower"}"#)
        .dispatch();

    assert_eq!(Status::Ok, res.status());
    res
}

#[test]
fn dig() {
    let (state, client) = setup();

    let response = enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(Status::Ok, response.status());

    let response = send_dig(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(Status::Ok, response.status());

    assert_eq!(DIGGING_RATE, state.get_current_state().get_intake().get_digger().get_speed());
    assert_eq!(DIGGING_RATE, state.get_intake().get_current_state().get_digger().get_speed());
}

#[test]
fn stop_digging() {
    let (state, client) = setup();
    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_dig(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_stop_digging(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, state.get_current_state().get_intake().get_digger().get_speed());
    assert_eq!(0.0, state.get_intake().get_current_state().get_digger().get_speed());
}

#[test]
fn stop_raise_actuators() {
    let (state, client) = setup();
    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_raise(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_stop_actuators(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, state.get_current_state().get_intake().get_digger().get_speed());
    assert_eq!(0.0, state.get_intake().get_current_state().get_digger().get_speed());
}


#[test]
fn stop_lower_actuators() {
    let (state, client) = setup();
    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_lower(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_stop_actuators(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, state.get_current_state().get_intake().get_digger().get_speed());
    assert_eq!(0.0, state.get_intake().get_current_state().get_digger().get_speed());
}

#[test]
fn upper_left() {
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

    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    left_input.store(true, Ordering::SeqCst);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_raise(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(true, left_limit.load(Ordering::SeqCst));
    assert_eq!(false, right_limit.load(Ordering::SeqCst));
}

#[test]
fn upper_right() {
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

    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    right_input.store(true, Ordering::SeqCst);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_raise(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(false, left_limit.load(Ordering::SeqCst));
    assert_eq!(true, right_limit.load(Ordering::SeqCst));
}

#[test]
fn upper_both() {
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

    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    left_input.store(true, Ordering::SeqCst);
    right_input.store(true, Ordering::SeqCst);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_raise(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(true, left_limit.load(Ordering::SeqCst));
    assert_eq!(true, right_limit.load(Ordering::SeqCst));
}

#[test]
fn upper_none() {
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

    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_raise(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(MH_ACTUATOR_RATE, state.get_intake().get_actuator().get_speed());
    assert_eq!(false, left_limit.load(Ordering::SeqCst));
    assert_eq!(false, right_limit.load(Ordering::SeqCst));
}

#[test]
fn lower_left() {
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

    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    left_input.store(true, Ordering::SeqCst);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_lower(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(true, left_limit.load(Ordering::SeqCst));
    assert_eq!(false, right_limit.load(Ordering::SeqCst));
}

#[test]
fn lower_right() {
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

    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    right_input.store(true, Ordering::SeqCst);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_lower(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(false, left_limit.load(Ordering::SeqCst));
    assert_eq!(true, right_limit.load(Ordering::SeqCst));
}

#[test]
fn lower_both() {
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

    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    left_input.store(true, Ordering::SeqCst);
    right_input.store(true, Ordering::SeqCst);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_lower(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, actuator.get_speed());
    assert_eq!(true, left_limit.load(Ordering::SeqCst));
    assert_eq!(true, right_limit.load(Ordering::SeqCst));
}

#[test]
fn lower_none() {
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

    enable_intake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    send_lower(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(false, left_limit.load(Ordering::SeqCst));
    assert_eq!(false, right_limit.load(Ordering::SeqCst));
    assert_eq!(-MH_ACTUATOR_RATE, state.get_current_state().get_intake().get_actuator().get_speed());
    assert_eq!(-MH_ACTUATOR_RATE, state.get_intake().get_current_state().get_actuator().get_speed());
    assert_eq!(-MH_ACTUATOR_RATE, state.get_intake().get_actuator().get_current_state().get_speed());
}