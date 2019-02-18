use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;

use rocket::http::ContentType;
use rocket::local::Client;

use crate::comms;
use crate::status::life::GlobalLifeState;

use super::*;
use crate::mechatronics::commands::BrakeCommand;
use crate::mechatronics::commands::RobotCommand;
use crate::mechatronics::commands::ResetDumperCommand;
use crate::mechatronics::commands::DumpCommand;

struct TestEnvironment {
    receiver: Receiver<Box<RobotCommand>>,
    client: Client,
    status: Arc<GlobalRobotState>,
}

fn setup() -> TestEnvironment {
    let (controller_sender, controller_receiver) = channel();

    // Create Robot status
    let robot_status = Arc::new(GlobalRobotState::new());

    // Create RobotView
    let robot_view = MechatronicsMessageSender::new(controller_sender);

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