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

const TIMEOUT: u64 = 100;

struct TestMotorGroup {
    pub inverted: Arc<RwLock<bool>>,
    pub speed: Arc<RwLock<f32>>,
    pub motor_group: MotorGroup,
}

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

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());
    assert_eq!(0.0, *digger.speed.read().unwrap());
    assert_eq!(0.0, *rails.speed.read().unwrap());
    assert_eq!(0.0, *dumper.speed.read().unwrap());
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

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());
    assert_eq!(0.0, *digger.speed.read().unwrap());
    assert_eq!(0.0, *rails.speed.read().unwrap());
    assert_eq!(0.0, *dumper.speed.read().unwrap());
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

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());
    assert_eq!(DIGGING_RATE, *digger.speed.read().unwrap());
    assert_eq!(0.0, *rails.speed.read().unwrap());
    assert_eq!(0.0, *dumper.speed.read().unwrap());
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

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());
    assert_eq!(0.0, *digger.speed.read().unwrap());
    assert_eq!(MH_ACTUATOR_RATE, *rails.speed.read().unwrap());
    assert_eq!(0.0, *dumper.speed.read().unwrap());
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

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());
    assert_eq!(0.0, *digger.speed.read().unwrap());
    assert_eq!(-MH_ACTUATOR_RATE, *rails.speed.read().unwrap());
    assert_eq!(0.0, *dumper.speed.read().unwrap());
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

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());
    assert_eq!(0.0, *digger.speed.read().unwrap());
    assert_eq!(0.0, *rails.speed.read().unwrap());
    assert_eq!(DUMPING_RATE, *dumper.speed.read().unwrap());
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

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());
    assert_eq!(0.0, *digger.speed.read().unwrap());
    assert_eq!(0.0, *rails.speed.read().unwrap());
    assert_eq!(DUMPER_RESET_RATE, *dumper.speed.read().unwrap());
}

fn create_groups() -> (TestMotorGroup, TestMotorGroup) {
    let inverted_0 = Arc::new(RwLock::new(false));
    let inverted_1 = Arc::new(RwLock::new(false));

    let speed_0 = Arc::new(RwLock::new(0.0));
    let speed_1 = Arc::new(RwLock::new(0.0));

    let test_motor_0 = TestMotor::new(inverted_0.clone(), speed_0.clone());
    let test_motor_1 = TestMotor::new(inverted_1.clone(), speed_1.clone());

    let test_group_0 = MotorGroup::new(vec![Box::new(test_motor_0)]);
    let test_group_1 = MotorGroup::new(vec![Box::new(test_motor_1)]);

    let test_unit_0 = TestMotorGroup { inverted: inverted_0, speed: speed_0, motor_group: test_group_0 };
    let test_unit_1 = TestMotorGroup { inverted: inverted_1, speed: speed_1, motor_group: test_group_1 };

    (test_unit_0, test_unit_1)
}