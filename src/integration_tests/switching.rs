use super::*;

#[test]
fn drive() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let response = client.post("/robot/modes/drive").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(true, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn dump() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let response = client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(true, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn dig() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let response = client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(true, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_dig_to_drive() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let _response = client.post("/robot/modes/dig").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/modes/drive").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(true, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_dig_to_dump() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let _response = client.post("/robot/modes/dig").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(true, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_drive_to_dump() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let _response = client.post("/robot/modes/drive").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/modes/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(true, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_drive_to_dig() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let _response = client.post("/robot/modes/drive").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(true, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_dump_to_drive() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let _response = client.post("/robot/modes/dump").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/modes/drive").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(true, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_dump_to_dig() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let _response = client.post("/robot/modes/dump").dispatch();

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(true, state.get_intake().get_current_state().get_enabled());
}