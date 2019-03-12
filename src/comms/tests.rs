use std::sync::mpsc::Receiver;
use std::sync::mpsc::sync_channel;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::{Client, LocalResponse};

use crate::comms;
use crate::mechatronics::commands::RobotCommand;

use super::*;

struct TestEnvironment {
    receiver: Receiver<Box<RobotCommand>>,
    client: Client,
    status: Arc<GlobalRobotState>,
}

impl TestEnvironment {
    pub fn send_drive(&self, left: f32, right: f32) -> LocalResponse {
        self.client.put("/robot/drive")
            .header(ContentType::JSON)
            .body(format!("{{ 'drive' : {{ 'left': {}, 'right': {} }} }}", left, right))
            .dispatch()
    }
}

fn setup() -> TestEnvironment {
    let (controller_sender, controller_receiver) = sync_channel(20);

    // Create Robot status
    let robot_status = Arc::new(GlobalRobotState::new());

    // Create RobotView
    let robot_view = RobotMessenger::new(controller_sender);

    // Create server
    let grasshopper = comms::stage(robot_view, robot_status.clone(), RobotCommandFactory::new());

    let client = Client::new(grasshopper).unwrap();

    TestEnvironment {
        receiver: controller_receiver,
        client,
        status: robot_status,
    }
}

#[test]
fn test_bad_switch() {
    let env = setup();
    let response = env.client
        .put("/robot")
        .header(ContentType::JSON)
        .body(r#"{ "mode" : "invalid" }"#)
        .dispatch();
    assert_eq!(Status::UnprocessableEntity, response.status());
}

#[test]
fn test_bad_drive() {
    let env = setup();
    let response = env.send_drive(2.0, 1.0);
    assert_eq!(Status::BadRequest, response.status());
    let response = env.send_drive(1.0, 2.0);
    assert_eq!(Status::BadRequest, response.status());
    let response = env.send_drive(2.0, 2.0);
    assert_eq!(Status::BadRequest, response.status());
    let env = setup();
    let response = env.send_drive(-2.0, 1.0);
    assert_eq!(Status::BadRequest, response.status());
    let response = env.send_drive(1.0, -2.0);
    assert_eq!(Status::BadRequest, response.status());
    let response = env.send_drive(-2.0, -2.0);
    assert_eq!(Status::BadRequest, response.status());
}

#[test]
fn test_state() {
    let env = setup();
    let mut response = env.client.get("/robot").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert!(response.body().is_some());
}

#[test]
fn test_index() {
    let env = setup();
    let mut response = env.client.get("/").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert!(response.body().is_some());
}

#[test]
fn favicon() {
    let env = setup();
    let mut response = env.client.get("/favicon.ico").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert!(response.body().is_some());
}

#[test]
fn test_file() {
    let env = setup();
    let mut response = env.client.get("/static/main.css").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert!(response.body().is_some());
}