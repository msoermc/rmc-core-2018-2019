use rocket::local::Client;

use crate::comms;
use crate::mechatronics::MechatronicsCommand;
use crate::mechatronics::RobotLifeStatus;

use super::*;
use crate::mechatronics::GlobalLifeStatus;

struct TestEnvironment {
    receiver: Receiver<MechatronicsCommand>,
    sender: ServerSender,
    client: Client,
    status: GlobalLifeStatus
}

fn setup() -> TestEnvironment {
    let (controller_sender, controller_receiver) = channel();

    // Create Robot status
    let robot_status = GlobalLifeStatus::new();

    // Create RobotView
    let robot_view = MechatronicsMessageSender::new(controller_sender, robot_status.clone());

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
    env.status.revive();
    let response = env.client.post("/robot/kill").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(RobotLifeStatus::Dead, env.status.get_status());
}

#[test]
fn test_revive() {
    let env = setup();
    env.status.kill();
    let response = env.client.post("/robot/revive").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(RobotLifeStatus::Alive, env.status.get_status());
}

#[test]
fn test_enable_drive() {
    let env = setup();
    let response = env.client.post("/robot/drive_train/enable").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::EnableDrive, env.receiver.try_recv().unwrap());
}

#[test]
fn test_disable_drive() {
    let env = setup();
    let response = env.client.post("/robot/drive_train/disable").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::DisableDrive, env.receiver.try_recv().unwrap());
}

#[test]
fn test_brake() {
    let env = setup();
    let response = env.client.post("/robot/drive_train/brake").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::Brake, env.receiver.try_recv().unwrap());
}

#[test]
fn test_enable_dumper() {
    let env = setup();
    let response = env.client.post("/robot/dumper/enable").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::EnableDumper, env.receiver.try_recv().unwrap());
}

#[test]
fn test_disable_dumper() {
    let env = setup();
    let response = env.client.post("/robot/dumper/disable").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::DisableDumper, env.receiver.try_recv().unwrap());
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
fn test_enable_digger() {
    let env = setup();
    let response = env.client.post("/robot/digger/enable").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::EnableBucketLadder, env.receiver.try_recv().unwrap());
}

#[test]
fn test_disable_digger() {
    let env = setup();
    let response = env.client.post("/robot/digger/disable").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::DisableBucketLadder, env.receiver.try_recv().unwrap());
}

#[test]
fn test_dig() {
    let env = setup();
    let response = env.client.post("/robot/digger/dig").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::Dig, env.receiver.try_recv().unwrap());
}

#[test]
fn test_stop_digger() {
    let env = setup();
    let response = env.client.post("/robot/digger/stop").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::StopDigging, env.receiver.try_recv().unwrap());
}

#[test]
fn test_raise_digger() {
    let env = setup();
    let response = env.client.post("/robot/digger/rails/raise").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::RaiseDigger, env.receiver.try_recv().unwrap());
}

#[test]
fn test_lower_digger() {
    let env = setup();
    let response = env.client.post("/robot/digger/rails/lower").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::LowerDigger, env.receiver.try_recv().unwrap());
}

#[test]
fn test_stop_rails() {
    let env = setup();
    let response = env.client.post("/robot/digger/rails/stop").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(MechatronicsCommand::FreezeDiggerHeight, env.receiver.try_recv().unwrap());
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