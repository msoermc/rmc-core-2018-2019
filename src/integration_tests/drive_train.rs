use rocket::http::ContentType;
use rocket::local::{Client, LocalResponse};

use crate::status::robot_state::GlobalRobotState;

use super::*;
use crate::logging::launch_logger;

const TIMEOUT_MILLIS: u64 = 30;

fn send_drive(client: &Client, left: f32, right: f32) -> LocalResponse {
    let json = format!("{{\"Drive\" : {{ \"left\": {}, \"right\": {} }} }}", left, right);

    println!("{}", json);
    client.put("/robot/drive")
        .header(ContentType::JSON)
        .body(json)
        .dispatch()
}

fn send_brake(client: &Client) -> LocalResponse {
    client.put("/robot/drive")
        .header(ContentType::JSON)
        .body(r#" "Brake" "#)
        .dispatch()
}

fn enable_drive(client: &Client) -> LocalResponse {
    client.put("/robot")
        .header(ContentType::JSON)
        .body(r#"{"mode":"Driving"}"#)
        .dispatch()
}

#[test]
fn drive() {
    let (state, client) = setup();
    enable_drive(&client);
    let response = send_drive(&client, 1.0, -1.0);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(Status::Ok, response.status());
    assert_eq!(1.0, state.get_current_state().get_drive().get_left().get_speed());
    assert_eq!(-1.0, state.get_current_state().get_drive().get_right().get_speed());
}

#[test]
fn brake() {
    let (state, client) = setup();
    enable_drive(&client);
    send_drive(&client, 1.0, 1.0);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = send_brake(&client);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(Status::Ok, response.status());

    assert_eq!(0.0, state.get_drive().get_current_state().get_left().get_speed());
    assert_eq!(0.0, state.get_drive().get_current_state().get_right().get_speed());
}