use std::sync::Arc;
use std::sync::RwLock;

use rocket::local::Client;

use crate::comms;
use crate::control::RobotControllerCommand;
use crate::control::RobotLifeStatus;

use super::*;

struct TestEnvironment {
    receiver: Receiver<RobotControllerCommand>,
    sender: ServerSender,
    client: Client,
    status: Arc<RwLock<RobotLifeStatus>>
}

fn setup() -> TestEnvironment {
    let (controller_sender, controller_receiver) = channel();

    // Create Robot status
    let robot_status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

    // Create RobotView
    let robot_view = RobotView::new(controller_sender, robot_status.clone());

    // Create server
    let (server_sender, grasshopper) = comms::stage(robot_view);

    let client = Client::new(grasshopper).unwrap();

    TestEnvironment {
        receiver: controller_receiver,
        sender: server_sender,
        client,
        status: robot_status
    }
}

#[test]
fn test_drive_request() {
    let env = setup();

    let response = env.client.post("/robot/drive_train/drive/1.0/1.0").dispatch();
    assert_eq!(Status::Ok, response.status());

    if let RobotControllerCommand::Drive(result) = env.receiver.recv().unwrap() {
        assert_eq!(1.0, result.get_left_speed());
        assert_eq!(1.0, result.get_right_speed());
    } else {
        panic!("Expected drive command, got {:?}!", )
    }

    // bad request
    let response = env.client.post("/robot/drive_train/drive/1.0/1.1").dispatch();
    assert_eq!(Status::BadRequest, response.status());
    assert!(env.receiver.try_recv().is_err());
}

#[test]
fn test_kill() {
    let env = setup();
    *env.status.write().unwrap() = RobotLifeStatus::Alive;
    let response = env.client.post("/robot/kill").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(RobotLifeStatus::Dead, *env.status.read().unwrap());
}

#[test]
fn test_revive() {
    let env = setup();
    *env.status.write().unwrap() = RobotLifeStatus::Dead;
    let response = env.client.post("/robot/revive").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(RobotLifeStatus::Alive, *env.status.read().unwrap());
}

#[test]
fn test_enable_drive() {
    let env = setup();
    let response = env.client.post("/robot/drive_train/enable").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(RobotControllerCommand::Enable, env.receiver.try_recv().unwrap());
}

#[test]
fn test_disable_drive() {
    let env = setup();
    let response = env.client.post("/robot/drive_train/disable").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(RobotControllerCommand::Disable, env.receiver.try_recv().unwrap());
}

#[test]
fn test_brake() {
    let env = setup();
    let response = env.client.post("/robot/drive_train/brake").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(RobotControllerCommand::Brake, env.receiver.try_recv().unwrap());
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