use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::robot_map::*;

use rocket::http::Status;

use super::*;
use crate::robot::RobotBuilder;
use std::time::Duration;
use std::thread::sleep;
use crate::devices::motor_controllers::GlobalMotorState;

const TIMEOUT: u64 = 100;

#[test]
fn test_setup() {
    let (left, right) = create_groups();
    let (digger, rails) = create_groups();
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);
    builder.use_custom_intake(digger.motor_group, rails.motor_group);
    builder.use_custom_dumper(dumper.motor_group);
    let _client = builder.build().launch_tester();

    assert_eq!(0.0, left.state.get_speed());
    assert_eq!(0.0, right.state.get_speed());
    assert_eq!(0.0, digger.state.get_speed());
    assert_eq!(0.0, rails.state.get_speed());
    assert_eq!(0.0, dumper.state.get_speed());
}

#[test]
fn test_drive() {
    let (left, right) = create_groups();
    let (digger, rails) = create_groups();
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);
    builder.use_custom_intake(digger.motor_group, rails.motor_group);
    builder.use_custom_dumper(dumper.motor_group);
    let client = builder.build().launch_tester();

    client.post("/robot/drive_train/drive/1/1").dispatch();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(1.0, left.state.get_speed());
    assert_eq!(1.0, right.state.get_speed());
    assert_eq!(0.0, digger.state.get_speed());
    assert_eq!(0.0, rails.state.get_speed());
    assert_eq!(0.0, dumper.state.get_speed());

    client.post("/robot/modes/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(0.0, left.state.get_speed());
    assert_eq!(0.0, right.state.get_speed());
    assert_eq!(0.0, digger.state.get_speed());
    assert_eq!(0.0, rails.state.get_speed());
    assert_eq!(0.0, dumper.state.get_speed());

    client.post("/robot/modes/drive").dispatch();
    client.post("/robot/drive_train/drive/1/1").dispatch();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(1.0, left.state.get_speed());
    assert_eq!(1.0, right.state.get_speed());
    assert_eq!(0.0, digger.state.get_speed());
    assert_eq!(0.0, rails.state.get_speed());
    assert_eq!(0.0, dumper.state.get_speed());
}

#[test]
fn test_dig() {
    let (left, right) = create_groups();
    let (digger, rails) = create_groups();
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);
    builder.use_custom_intake(digger.motor_group, rails.motor_group);
    builder.use_custom_dumper(dumper.motor_group);
    let client = builder.build().launch_tester();

    client.post("/robot/modes/dig").dispatch();
    client.post("/robot/intake/digger/dig").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(0.0, left.state.get_speed());
    assert_eq!(0.0, right.state.get_speed());
    assert_eq!(DIGGING_RATE, digger.state.get_speed());
    assert_eq!(0.0, rails.state.get_speed());
    assert_eq!(0.0, dumper.state.get_speed());
}

#[test]
fn test_raise() {
    let (left, right) = create_groups();
    let (digger, rails) = create_groups();
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);
    builder.use_custom_intake(digger.motor_group, rails.motor_group);
    builder.use_custom_dumper(dumper.motor_group);
    let client = builder.build().launch_tester();

    client.post("/robot/modes/dig").dispatch();
    client.post("/robot/intake/rails/raise").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(0.0, left.state.get_speed());
    assert_eq!(0.0, right.state.get_speed());
    assert_eq!(0.0, digger.state.get_speed());
    assert_eq!(MH_ACTUATOR_RATE, rails.state.get_speed());
    assert_eq!(0.0, dumper.state.get_speed());
}

#[test]
fn test_lower() {
    let (left, right) = create_groups();
    let (digger, rails) = create_groups();
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);
    builder.use_custom_intake(digger.motor_group, rails.motor_group);
    builder.use_custom_dumper(dumper.motor_group);
    let client = builder.build().launch_tester();

    client.post("/robot/modes/dig").dispatch();
    client.post("/robot/intake/rails/lower").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(0.0, left.state.get_speed());
    assert_eq!(0.0, right.state.get_speed());
    assert_eq!(0.0, digger.state.get_speed());
    assert_eq!(-MH_ACTUATOR_RATE, rails.state.get_speed());
    assert_eq!(0.0, dumper.state.get_speed());
}

#[test]
fn test_dump() {
    let (left, right) = create_groups();
    let (digger, rails) = create_groups();
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);
    builder.use_custom_intake(digger.motor_group, rails.motor_group);
    builder.use_custom_dumper(dumper.motor_group);
    let client = builder.build().launch_tester();

    client.post("/robot/modes/dump").dispatch();
    client.post("/robot/dumper/dump").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(0.0, left.state.get_speed());
    assert_eq!(0.0, right.state.get_speed());
    assert_eq!(0.0, digger.state.get_speed());
    assert_eq!(0.0, rails.state.get_speed());
    assert_eq!(DUMPING_RATE, dumper.state.get_speed());
}

#[test]
fn test_reset_dumper() {
    let (left, right) = create_groups();
    let (digger, rails) = create_groups();
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);
    builder.use_custom_intake(digger.motor_group, rails.motor_group);
    builder.use_custom_dumper(dumper.motor_group);
    let client = builder.build().launch_tester();

    client.post("/robot/modes/dump").dispatch();
    client.post("/robot/dumper/reset").dispatch();
    sleep(Duration::from_millis(TIMEOUT));

    assert_eq!(0.0, left.state.get_speed());
    assert_eq!(0.0, right.state.get_speed());
    assert_eq!(0.0, digger.state.get_speed());
    assert_eq!(0.0, rails.state.get_speed());
    assert_eq!(DUMPER_RESET_RATE, dumper.state.get_speed());
}

struct TestMotorGroup {
    pub state: Arc<GlobalMotorState>,
    pub motor_group: MotorGroup,
}

fn create_groups() -> (TestMotorGroup, TestMotorGroup) {
    let state_0 = Arc::new(GlobalMotorState::new());
    let state_1 = Arc::new(GlobalMotorState::new());

    let test_motor_0 = TestMotor::new(state_0.clone());
    let test_motor_1 = TestMotor::new(state_1.clone());

    let test_group_0 = MotorGroup::new(vec![Box::new(test_motor_0)]);
    let test_group_1 = MotorGroup::new(vec![Box::new(test_motor_1)]);

    let test_unit_0 = TestMotorGroup { state: state_0, motor_group: test_group_0 };
    let test_unit_1 = TestMotorGroup { state: state_1, motor_group: test_group_1 };

    (test_unit_0, test_unit_1)
}