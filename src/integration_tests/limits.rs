use std::sync::atomic::AtomicBool;

use atomic::Ordering;

use crate::pinouts::digital::TestPin;

use super::*;

const TIMEOUT_MILLIS: u64 = 50;

//#[test]
//fn upper_left() {
//    let left_input = Arc::new(AtomicBool::new(false));
//    let right_input = Arc::new(AtomicBool::new(false));
//
//    let left_pin = Box::new(TestPin::new(left_input.clone()));
//    let right_pin = Box::new(TestPin::new(right_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_upper_left_intake_limit(left_pin);
//    builder.with_test_upper_right_intake_limit(right_pin);
//
//    let state = builder.get_state();
//
//    let actuator = state.get_intake().get_actuator();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let left_limit = state.get_intake().get_left_actuator().get_upper();
//    let right_limit = state.get_intake().get_right_actuator().get_upper();
//
//    client.post("/robot/modes/dig").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    left_input.store(true, Ordering::SeqCst);
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/intake/rails/raise").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, actuator.get_speed());
//    assert_eq!(true, left_limit.load(Ordering::SeqCst));
//    assert_eq!(false, right_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn upper_right() {
//    let left_input = Arc::new(AtomicBool::new(false));
//    let right_input = Arc::new(AtomicBool::new(false));
//
//    let left_pin = Box::new(TestPin::new(left_input.clone()));
//    let right_pin = Box::new(TestPin::new(right_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_upper_left_intake_limit(left_pin);
//    builder.with_test_upper_right_intake_limit(right_pin);
//
//    let state = builder.get_state();
//
//    let actuator = state.get_intake().get_actuator();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let left_limit = state.get_intake().get_left_actuator().get_upper();
//    let right_limit = state.get_intake().get_right_actuator().get_upper();
//
//    client.post("/robot/modes/dig").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    right_input.store(true, Ordering::SeqCst);
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/intake/rails/raise").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, actuator.get_speed());
//    assert_eq!(false, left_limit.load(Ordering::SeqCst));
//    assert_eq!(true, right_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn upper_both() {
//    let left_input = Arc::new(AtomicBool::new(false));
//    let right_input = Arc::new(AtomicBool::new(false));
//
//    let left_pin = Box::new(TestPin::new(left_input.clone()));
//    let right_pin = Box::new(TestPin::new(right_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_upper_left_intake_limit(left_pin);
//    builder.with_test_upper_right_intake_limit(right_pin);
//
//    let state = builder.get_state();
//
//    let actuator = state.get_intake().get_actuator();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let left_limit = state.get_intake().get_left_actuator().get_upper();
//    let right_limit = state.get_intake().get_right_actuator().get_upper();
//
//    client.post("/robot/modes/dig").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    left_input.store(true, Ordering::SeqCst);
//    right_input.store(true, Ordering::SeqCst);
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/intake/rails/raise").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, actuator.get_speed());
//    assert_eq!(true, left_limit.load(Ordering::SeqCst));
//    assert_eq!(true, right_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn upper_none() {
//    let left_input = Arc::new(AtomicBool::new(false));
//    let right_input = Arc::new(AtomicBool::new(false));
//
//    let left_pin = Box::new(TestPin::new(left_input.clone()));
//    let right_pin = Box::new(TestPin::new(right_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_upper_left_intake_limit(left_pin);
//    builder.with_test_upper_right_intake_limit(right_pin);
//
//    let state = builder.get_state();
//
//    let actuator = state.get_intake().get_actuator();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let left_limit = state.get_intake().get_left_actuator().get_upper();
//    let right_limit = state.get_intake().get_right_actuator().get_upper();
//
//    client.post("/robot/modes/dig").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/intake/rails/raise").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(MH_ACTUATOR_RATE, state.get_intake().get_actuator().get_speed());
//    assert_eq!(false, left_limit.load(Ordering::SeqCst));
//    assert_eq!(false, right_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn lower_left() {
//    let left_input = Arc::new(AtomicBool::new(false));
//    let right_input = Arc::new(AtomicBool::new(false));
//
//    let left_pin = Box::new(TestPin::new(left_input.clone()));
//    let right_pin = Box::new(TestPin::new(right_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_lower_left_intake_limit(left_pin);
//    builder.with_test_lower_right_intake_limit(right_pin);
//
//    let state = builder.get_state();
//
//    let actuator = state.get_intake().get_actuator();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let left_limit = state.get_intake().get_left_actuator().get_lower();
//    let right_limit = state.get_intake().get_right_actuator().get_lower();
//
//    client.post("/robot/modes/dig").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    left_input.store(true, Ordering::SeqCst);
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/intake/rails/lower").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, actuator.get_speed());
//    assert_eq!(true, left_limit.load(Ordering::SeqCst));
//    assert_eq!(false, right_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn lower_right() {
//    let left_input = Arc::new(AtomicBool::new(false));
//    let right_input = Arc::new(AtomicBool::new(false));
//
//    let left_pin = Box::new(TestPin::new(left_input.clone()));
//    let right_pin = Box::new(TestPin::new(right_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_lower_left_intake_limit(left_pin);
//    builder.with_test_lower_right_intake_limit(right_pin);
//
//    let state = builder.get_state();
//
//    let actuator = state.get_intake().get_actuator();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let left_limit = state.get_intake().get_left_actuator().get_lower();
//    let right_limit = state.get_intake().get_right_actuator().get_lower();
//
//    client.post("/robot/modes/dig").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    right_input.store(true, Ordering::SeqCst);
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/intake/rails/lower").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, actuator.get_speed());
//    assert_eq!(false, left_limit.load(Ordering::SeqCst));
//    assert_eq!(true, right_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn lower_both() {
//    let left_input = Arc::new(AtomicBool::new(false));
//    let right_input = Arc::new(AtomicBool::new(false));
//
//    let left_pin = Box::new(TestPin::new(left_input.clone()));
//    let right_pin = Box::new(TestPin::new(right_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_lower_left_intake_limit(left_pin);
//    builder.with_test_lower_right_intake_limit(right_pin);
//
//    let state = builder.get_state();
//
//    let actuator = state.get_intake().get_actuator();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let left_limit = state.get_intake().get_left_actuator().get_lower();
//    let right_limit = state.get_intake().get_right_actuator().get_lower();
//
//    client.post("/robot/modes/dig").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    left_input.store(true, Ordering::SeqCst);
//    right_input.store(true, Ordering::SeqCst);
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/intake/rails/lower").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, actuator.get_speed());
//    assert_eq!(true, left_limit.load(Ordering::SeqCst));
//    assert_eq!(true, right_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn lower_none() {
//    let left_input = Arc::new(AtomicBool::new(false));
//    let right_input = Arc::new(AtomicBool::new(false));
//
//    let left_pin = Box::new(TestPin::new(left_input.clone()));
//    let right_pin = Box::new(TestPin::new(right_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_lower_left_intake_limit(left_pin);
//    builder.with_test_lower_right_intake_limit(right_pin);
//
//    let state = builder.get_state();
//
//    let actuator = state.get_intake().get_actuator();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let left_limit = state.get_intake().get_left_actuator().get_lower();
//    let right_limit = state.get_intake().get_right_actuator().get_lower();
//
//    client.post("/robot/modes/dig").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/intake/rails/lower").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(false, left_limit.load(Ordering::SeqCst));
//    assert_eq!(false, right_limit.load(Ordering::SeqCst));
//    assert_eq!(-MH_ACTUATOR_RATE, actuator.get_speed());
//}
//
//#[test]
//fn upper_dumper_tripped() {
//    let upper_input = Arc::new(AtomicBool::new(false));
//
//    let upper_pin = Box::new(TestPin::new(upper_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_upper_dumper_limit(upper_pin);
//
//    let state = builder.get_state();
//
//    let dumper = state.get_dumper().get_motor();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let upper_limit = state.get_dumper().get_upper_limit();
//
//    client.post("/robot/modes/dump").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    upper_input.store(true, Ordering::SeqCst);
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/dumper/dump").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, dumper.get_speed());
//    assert_eq!(true, upper_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn upper_dumper_not_tripped() {
//    let upper_input = Arc::new(AtomicBool::new(false));
//
//    let upper_pin = Box::new(TestPin::new(upper_input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_upper_dumper_limit(upper_pin);
//
//    let state = builder.get_state();
//
//    let dumper = state.get_dumper().get_motor();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let upper_limit = state.get_dumper().get_upper_limit();
//
//    client.post("/robot/modes/dump").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/dumper/dump").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(DUMPING_RATE, dumper.get_speed());
//    assert_eq!(false, upper_limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn lower_dumper_tripped() {
//    let input = Arc::new(AtomicBool::new(false));
//
//    let pin = Box::new(TestPin::new(input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_lower_dumper_limit(pin);
//
//    let state = builder.get_state();
//
//    let dumper = state.get_dumper().get_motor();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let limit = state.get_dumper().get_lower_limit();
//
//    client.post("/robot/modes/reset").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    input.store(true, Ordering::SeqCst);
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, dumper.get_speed());
//    assert_eq!(true, limit.load(Ordering::SeqCst));
//
//    client.post("/robot/dumper/reset").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(0.0, dumper.get_speed());
//    assert_eq!(true, limit.load(Ordering::SeqCst));
//}
//
//#[test]
//fn lower_dumper_not_tripped() {
//    let input = Arc::new(AtomicBool::new(false));
//
//    let pin = Box::new(TestPin::new(input.clone()));
//
//    let mut builder = RobotAssemblyBuilder::new();
//
//    builder.with_test();
//
//    builder.with_test_lower_dumper_limit(pin);
//
//    let state = builder.get_state();
//
//    let dumper = state.get_dumper().get_motor();
//
//    let client = builder.generate().assemble().launch().engage_testing_server();
//
//    let limit = state.get_dumper().get_lower_limit();
//
//    client.post("/robot/modes/dump").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    client.post("/robot/dumper/reset").dispatch();
//    sleep(Duration::from_millis(TIMEOUT_MILLIS));
//
//    assert_eq!(DUMPER_RESET_RATE, dumper.get_speed());
//    assert_eq!(false, limit.load(Ordering::SeqCst));
//}