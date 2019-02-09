use super::*;

#[test]
fn drive() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    let response = client.post("/robot/modes/drive").dispatch();
    assert_eq!(Status::Ok, response.status());

    sleep(Duration::from_millis(TIMEOUT));

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
    assert_eq!(Status::Ok, response.status());
    sleep(Duration::from_millis(TIMEOUT));

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
    assert_eq!(Status::Ok, response.status());
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(true, state.get_intake().get_current_state().get_enabled());
}