use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;

use rocket::local::Client;

use crate::comms;
use crate::mechatronics::commands::RobotCommand;

use super::*;
use std::sync::mpsc::sync_channel;

struct TestEnvironment {
    receiver: Receiver<Box<RobotCommand>>,
    client: Client,
    status: Arc<GlobalRobotState>,
}

fn setup() -> TestEnvironment {
    let (controller_sender, controller_receiver) = sync_channel(20);

    // Create Robot status
    let robot_status = Arc::new(Default::default());

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
    let response = env.client.post("/robot/modes/invalid").dispatch();
    assert_eq!(Status::BadRequest, response.status());
}

#[test]
fn test_bad_drive() {
    let env = setup();
    let response = env.client.post("/robot/drive_train/drive/2/1").dispatch();
    assert_eq!(Status::BadRequest, response.status());
    let response = env.client.post("/robot/drive_train/drive/1/2").dispatch();
    assert_eq!(Status::BadRequest, response.status());
    let response = env.client.post("/robot/drive_train/drive/2/2").dispatch();
    assert_eq!(Status::BadRequest, response.status());
    let env = setup();
    let response = env.client.post("/robot/drive_train/drive/-2/1").dispatch();
    assert_eq!(Status::BadRequest, response.status());
    let response = env.client.post("/robot/drive_train/drive/1/-2").dispatch();
    assert_eq!(Status::BadRequest, response.status());
    let response = env.client.post("/robot/drive_train/drive/-2/-2").dispatch();
    assert_eq!(Status::BadRequest, response.status());
}

#[test]
fn test_state() {
    let env = setup();
    let mut response = env.client.get("/robot/state").dispatch();
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
fn test_file() {
    let env = setup();
    let mut response = env.client.get("/static/main.css").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert!(response.body().is_some());
}