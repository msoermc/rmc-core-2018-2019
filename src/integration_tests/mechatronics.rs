use super::*;


#[test]
fn test_drive() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    client.post("/robot/modes/drive").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    let response = client.post("/robot/drive_train/drive/1/-1").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(1.0, state.get_current_state().get_drive().get_left().get_speed());
    assert_eq!(-1.0, state.get_current_state().get_drive().get_right().get_speed());
}

#[test]
fn dig() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    let response = client.post("/robot/intake/digger/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(DIGGING_RATE, state.get_current_state().get_intake().get_ladder().get_motor().get_speed());
}

#[test]
fn stop_digger() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let client = robot.launch_tester();

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    let _response = client.post("/robot/intake/digger/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    let response = client.post("/robot/intake/digger/stop").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(0.0, state.get_current_state().get_intake().get_ladder().get_motor().get_speed());
}