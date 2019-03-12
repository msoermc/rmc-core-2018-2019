use rocket::http::ContentType;
use rocket::local::LocalResponse;
use rocket_contrib::json::Json;
use serde::ser::Serialize;

use crate::comms::RobotMode;

use super::*;
use crate::logging::launch_logger;

const TIMEOUT_MILLIS: u64 = 50;

fn switch(client: &Client, mode: RobotMode) -> LocalResponse {
    client.post("/robot/mode")
        .header(ContentType::JSON)
        .body(match mode {
            RobotMode::Digging => r#""Digging""#,
            RobotMode::Driving => r#""Driving""#,
            RobotMode::Dumping => r#""Dumping""#,
        })
        .dispatch()
}

#[test]
fn drive() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let response = switch(&client, RobotMode::Driving);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
    assert_eq!(true, state.get_drive().get_current_state().get_enabled());
}

#[test]
fn dump() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let response = switch(&client, RobotMode::Dumping);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(true, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn dig() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let response = switch(&client, RobotMode::Digging);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(true, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_dig_to_drive() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let _response = switch(&client, RobotMode::Digging);

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = switch(&client, RobotMode::Driving);

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(true, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_dig_to_dump() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let _response = switch(&client, RobotMode::Digging);

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = switch(&client, RobotMode::Dumping);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(true, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_drive_to_dump() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let _response = switch(&client, RobotMode::Driving);

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = switch(&client, RobotMode::Dumping);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(true, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_drive_to_dig() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let _response = switch(&client, RobotMode::Driving);

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = switch(&client, RobotMode::Digging);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(true, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_dump_to_drive() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let _response = switch(&client, RobotMode::Dumping);

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = switch(&client, RobotMode::Driving);

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(true, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn switch_dump_to_dig() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    let _response = switch(&client, RobotMode::Dumping);

    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = switch(&client, RobotMode::Digging);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(Status::Ok, response.status());
    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(true, state.get_intake().get_current_state().get_enabled());
}