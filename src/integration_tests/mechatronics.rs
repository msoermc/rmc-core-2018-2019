use super::*;

const TIMEOUT_MILLIS: u64 = 30;

#[test]
fn test_drive() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/drive").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/drive_train/drive/1/-1").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(1.0, state.get_current_state().get_drive().get_left().get_speed());
    assert_eq!(-1.0, state.get_current_state().get_drive().get_right().get_speed());
}

#[test]
fn test_kill_drive() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/drive").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post("/robot/drive_train/drive/1/-1").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/kill").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_drive().get_left().get_speed());
    assert_eq!(0.0, state.get_current_state().get_drive().get_right().get_speed());
}

#[test]
fn test_brake() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/drive").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/drive_train/drive/1/-1").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/drive_train/brake").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_drive().get_left().get_speed());
    assert_eq!(0.0, state.get_current_state().get_drive().get_right().get_speed());
}

#[test]
fn dig() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/digger/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(DIGGING_RATE, state.get_current_state().get_intake().get_digger().get_speed());
}

#[test]
fn stop_digger() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let _response = client.post("/robot/intake/digger/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/digger/stop").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_intake().get_digger().get_speed());
}

#[test]
fn kill_digger() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let _response = client.post("/robot/intake/digger/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/kill").dispatch();
    sleep(Duration::from_millis(50));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_intake().get_digger().get_speed());
}

#[test]
fn raise() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/rails/raise").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(MH_ACTUATOR_RATE, state.get_current_state().get_intake().get_actuator().get_speed());
}

#[test]
fn lower() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/rails/lower").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(-MH_ACTUATOR_RATE, state.get_current_state().get_intake().get_actuator().get_speed());
}

#[test]
fn stop_actuators() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/rails/lower").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/rails/stop").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_intake().get_actuator().get_speed());

    let response = client.post("/robot/intake/rails/raise").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/rails/stop").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_intake().get_actuator().get_speed());
}

#[test]
fn kill_actuators() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/rails/lower").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/kill").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_intake().get_actuator().get_speed());

    let response = client.post("/robot/revive").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/intake/rails/raise").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/kill").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_intake().get_actuator().get_speed());
}

#[test]
fn dump() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/dumper/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(DUMPING_RATE, state.get_current_state().get_dumper().get_motor().get_speed());
}

#[test]
fn reset_dumper() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/dumper/reset").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(DUMPER_RESET_RATE, state.get_current_state().get_dumper().get_motor().get_speed());
}

#[test]
fn stop_dumper() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let _response = client.post("/robot/dumper/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/dumper/stop").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_dumper().get_motor().get_speed());

    let _response = client.post("/robot/dumper/reset").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/dumper/stop").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_dumper().get_motor().get_speed());
}

#[test]
fn kill_dumper() {
    const TIMEOUT_MILLIS: u64 = 100;
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let _response = client.post("/robot/dumper/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/kill").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_dumper().get_motor().get_speed());

    let response = client.post("/robot/revive").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let _response = client.post("/robot/dumper/reset").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(DUMPER_RESET_RATE, state.get_current_state().get_dumper().get_motor().get_speed());

    let response = client.post("/robot/dumper/stop").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_dumper().get_motor().get_speed());
}