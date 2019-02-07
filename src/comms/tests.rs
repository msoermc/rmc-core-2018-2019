use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;

use rocket::http::ContentType;
use rocket::local::Client;

use crate::comms;
use crate::mechatronics::MechatronicsCommand;
use crate::status::life::GlobalLifeState;

use super::*;

struct TestEnvironment {
    receiver: Receiver<MechatronicsCommand>,
    client: Client,
    status: Arc<GlobalRobotState>,
}

fn setup() -> TestEnvironment {
    let (controller_sender, controller_receiver) = channel();

    // Create Robot status
    let robot_status = Arc::new(GlobalRobotState::new());

    // Create RobotView
    let robot_view = MechatronicsMessageSender::new(controller_sender, robot_status.clone());

    // Create server
    let grasshopper = comms::stage(robot_view, robot_status.clone());

    let client = Client::new(grasshopper).unwrap();

    TestEnvironment {
        receiver: controller_receiver,
        client,
        status: robot_status,
    }
}

#[test]
fn test_get_state() {
    let env = setup();

    let response = env.client.get("/robot/state").dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(Some(ContentType::JSON), response.content_type());
}

#[test]
fn test_lower() {
    let env = setup();

    let response = env.client.post("/robot/intake/rails/lower").dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::LowerActuators, env.receiver.try_recv().unwrap());
}

#[test]
fn test_raise() {
    let env = setup();

    let response = env.client.post("/robot/intake/rails/raise").dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::RaiseActuators, env.receiver.try_recv().unwrap());
}

#[test]
fn test_drive_request() {
    let env = setup();

    let response = env.client.post("/robot/drive_train/drive/1.0/1.0").dispatch();
    assert_eq!(Status::Ok, response.status());

    if let MechatronicsCommand::Drive(result) = env.receiver.recv().unwrap() {
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
    env.status.get_life().revive();
    let response = env.client.post("/robot/kill").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert!(!env.status.get_life().is_alive());
}

#[test]
fn test_revive() {
    let env = setup();
    env.status.get_life().kill();
    let response = env.client.post("/robot/revive").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert!(env.status.get_life().is_alive());
}

#[test]
fn test_brake() {
    let env = setup();
    let response = env.client.post("/robot/drive_train/brake").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::Brake, env.receiver.try_recv().unwrap());
}

#[test]
fn test_dump() {
    let env = setup();
    let response = env.client.post("/robot/dumper/dump").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::Dump, env.receiver.try_recv().unwrap());
}

#[test]
fn test_reset_dumper() {
    let env = setup();
    let response = env.client.post("/robot/dumper/reset").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::ResetDumper, env.receiver.try_recv().unwrap());
}

#[test]
fn test_stop_dumper() {
    let env = setup();
    let response = env.client.post("/robot/dumper/stop").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::StopDumper, env.receiver.try_recv().unwrap());
}

#[test]
fn test_dig() {
    let env = setup();
    let response = env.client.post("/robot/intake/digger/dig").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::Dig, env.receiver.try_recv().unwrap());
}

#[test]
fn test_bad_switch() {
    let env = setup();
    let response = env.client.post("/robot/modes/invalid").dispatch();
    assert_eq!(Status::BadRequest, response.status());
}

#[test]
fn test_drive_switch() {
    let env = setup();
    let response = env.client.post("/robot/modes/drive").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::EnterDriveMode, env.receiver.try_recv().unwrap());
}

#[test]
fn test_dig_switch() {
    let env = setup();
    let response = env.client.post("/robot/modes/dig").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::EnterDiggingMode, env.receiver.try_recv().unwrap());
}

#[test]
fn test_dump_switch() {
    let env = setup();
    let response = env.client.post("/robot/modes/dump").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::EnterDumpMode, env.receiver.try_recv().unwrap());
}

#[test]
fn test_stop_digger() {
    let env = setup();
    let response = env.client.post("/robot/intake/digger/stop").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::StopDigging, env.receiver.try_recv().unwrap());
}

#[test]
fn test_stop_rails() {
    let env = setup();
    let response = env.client.post("/robot/intake/rails/stop").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::StopActuators, env.receiver.try_recv().unwrap());
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