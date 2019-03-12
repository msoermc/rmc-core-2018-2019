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

fn get_enable_drive_url() -> String {
    "/robot/modes/drive".to_owned()
}

#[test]
fn drive() {
    let (state, client) = setup();
    client.post(get_enable_drive_url()).dispatch();
    client.post("/robot/drive_train/drive/1/-1").dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(1.0, state.get_drive().get_left().get_speed());
    assert_eq!(-1.0, state.get_drive().get_right().get_speed());
}

#[test]
fn brake() {
    let (state, client) = setup();
    client.post(get_enable_drive_url());
    client.post(get_drive_url(1.0, 1.0)).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_brake_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, state.get_drive().get_left().get_speed());
    assert_eq!(0.0, state.get_drive().get_right().get_speed());
}