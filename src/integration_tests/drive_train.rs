use rocket::local::Client;

use crate::status::robot_state::GlobalRobotState;

use super::*;

const TIMEOUT_MILLIS: u64 = 30;

fn get_drive_url(left: f32, right: f32) -> String {
    format!("/robot/drive_train/drive/{}/{}", left, right)
}

fn get_brake_url() -> String {
    "/robot/drive_train/brake".to_owned()
}

fn setup() -> (Arc<GlobalRobotState>, Client) {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    client.post("/robot/modes/drive").dispatch();

    (state, client)
}

#[test]
fn drive_from_stopped() {
    let (state, client) = setup();
    client.post("/robot/drive_train/drive/1/-1").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(1.0, state.get_drive().get_left().get_speed());
    assert_eq!(-1.0, state.get_drive().get_right().get_speed());
}

#[test]
fn change_direction() {
    let (state, client) = setup();
    client.post(get_drive_url(1.0, 1.0)).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_drive_url(-1.0, -1.0)).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(-1.0, state.get_drive().get_left().get_speed());
    assert_eq!(-1.0, state.get_drive().get_right().get_speed());
}