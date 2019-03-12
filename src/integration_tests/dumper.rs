use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::pinouts::digital::TestPin;

const TIMEOUT_MILLIS: u64 = 30;

fn get_enable_dumper_url() -> String {
    "/robot/modes/dump".to_owned()
}

fn get_dump_url() -> String {
    "/robot/dumper/dump".to_owned()
}

fn get_reset_url() -> String {
    "/robot/dumper/reset".to_owned()
}

fn get_stop_url() -> String {
    "/robot/dumper/stop".to_owned()
}

#[test]
fn dump() {
    let (state, client) = setup();
    client.post(get_enable_dumper_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_dump_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(DUMPING_RATE, state.get_current_state().get_dumper().get_motor().get_speed());
    assert_eq!(DUMPING_RATE, state.get_dumper().get_current_state().get_motor().get_speed());
}

#[test]
fn reset() {
    let (state, client) = setup();
    client.post(get_enable_dumper_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_reset_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(DUMPER_RESET_RATE, state.get_current_state().get_dumper().get_motor().get_speed());
    assert_eq!(DUMPER_RESET_RATE, state.get_dumper().get_current_state().get_motor().get_speed());
}

#[test]
fn stop() {
    let (state, client) = setup();
    client.post(get_enable_dumper_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_dump_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_stop_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, state.get_current_state().get_dumper().get_motor().get_speed());
    assert_eq!(0.0, state.get_dumper().get_current_state().get_motor().get_speed());
}

#[test]
fn upper_dumper_tripped() {
    let upper_input = Arc::new(AtomicBool::new(false));

    let upper_pin = Box::new(TestPin::new(upper_input.clone()));

    let mut builder = RobotAssemblyBuilder::new();

    builder.with_test();

    builder.with_test_upper_dumper_limit(upper_pin);

    let state = builder.get_state();

    let dumper = state.get_dumper().get_motor();

    let client = builder.generate().assemble().launch().engage_testing_server();

    let upper_limit = state.get_dumper().get_upper_limit();

    client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    upper_input.store(true, Ordering::SeqCst);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post("/robot/dumper/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, dumper.get_speed());
    assert_eq!(true, upper_limit.load(Ordering::SeqCst));
}

#[test]
fn upper_dumper_not_tripped() {
    let upper_input = Arc::new(AtomicBool::new(false));

    let upper_pin = Box::new(TestPin::new(upper_input.clone()));

    let mut builder = RobotAssemblyBuilder::new();

    builder.with_test();

    builder.with_test_upper_dumper_limit(upper_pin);

    let state = builder.get_state();

    let dumper = state.get_dumper().get_motor();

    let client = builder.generate().assemble().launch().engage_testing_server();

    let upper_limit = state.get_dumper().get_upper_limit();

    client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post("/robot/dumper/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(DUMPING_RATE, dumper.get_speed());
    assert_eq!(false, upper_limit.load(Ordering::SeqCst));
}

#[test]
fn lower_dumper_tripped() {
    let input = Arc::new(AtomicBool::new(false));

    let pin = Box::new(TestPin::new(input.clone()));

    let mut builder = RobotAssemblyBuilder::new();

    builder.with_test();

    builder.with_test_lower_dumper_limit(pin);

    let state = builder.get_state();

    let dumper = state.get_dumper().get_motor();

    let client = builder.generate().assemble().launch().engage_testing_server();

    let limit = state.get_dumper().get_lower_limit();

    client.post("/robot/modes/reset").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    input.store(true, Ordering::SeqCst);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, dumper.get_speed());
    assert_eq!(true, limit.load(Ordering::SeqCst));

    client.post("/robot/dumper/reset").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, dumper.get_speed());
    assert_eq!(true, limit.load(Ordering::SeqCst));
}

#[test]
fn lower_dumper_not_tripped() {
    let input = Arc::new(AtomicBool::new(false));

    let pin = Box::new(TestPin::new(input.clone()));

    let mut builder = RobotAssemblyBuilder::new();

    builder.with_test();

    builder.with_test_lower_dumper_limit(pin);

    let state = builder.get_state();

    let dumper = state.get_dumper().get_motor();

    let client = builder.generate().assemble().launch().engage_testing_server();

    let limit = state.get_dumper().get_lower_limit();

    client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post("/robot/dumper/reset").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(DUMPER_RESET_RATE, dumper.get_speed());
    assert_eq!(false, limit.load(Ordering::SeqCst));
}