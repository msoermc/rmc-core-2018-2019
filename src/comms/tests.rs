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
        client
    }
}

#[test]
fn test_drive_request() {
    let env = setup();

    let response = env.client.post("/drive/1.0/1.0").dispatch();
    assert_eq!(Status::Ok, response.status());

    if let RobotControllerCommand::Drive(result) = env.receiver.recv().unwrap() {
        assert_eq!(1.0, result.left_speed);
        assert_eq!(1.0, result.right_speed);
    } else {
        panic!("Expected drive command, got {:?}!", )
    }

    // bad request
    let response = env.client.post("/drive/1.0/1.1").dispatch();
    assert_eq!(Status::BadRequest, response.status());
    assert!(env.receiver.try_recv().is_err());
}