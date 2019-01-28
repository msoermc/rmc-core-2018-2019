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

const TIMEOUT: u64 = 50;

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
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/drive_train/drive/1.0/1.0").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());
}

#[test]
fn test_raise_rails() {
    let (digger, rails) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_intake(digger.motor_group, rails.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/intake/rails/raise").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(MH_ACTUATOR_RATE, *rails.speed.read().unwrap());
}

#[test]
fn test_lower_rails() {
    let (digger, rails) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_intake(digger.motor_group, rails.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/intake/rails/lower").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(-MH_ACTUATOR_RATE, *rails.speed.read().unwrap());
}

#[test]
fn test_stop_rails() {
    let (digger, rails) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_intake(digger.motor_group, rails.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/intake/rails/stop").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *rails.speed.read().unwrap());
}

#[test]
fn test_dig() {
    let (digger, rails) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_intake(digger.motor_group, rails.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/intake/digger/dig").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(DIGGING_RATE, *digger.speed.read().unwrap());
}

#[test]
fn test_stop_digging() {
    let (digger, rails) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_intake(digger.motor_group, rails.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/intake/digger/stop").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *digger.speed.read().unwrap());
}

#[test]
fn test_dump() {
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_dumper(dumper.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/dumper/dump").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(DUMPING_RATE, *dumper.speed.read().unwrap());
}

#[test]
fn test_reset_dumper() {
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_dumper(dumper.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/dumper/reset").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(DUMPER_RESET_RATE, *dumper.speed.read().unwrap());
}

#[test]
fn test_stop_dumper() {
    let (_, dumper) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_dumper(dumper.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/dumper/stop").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *dumper.speed.read().unwrap());
}

#[test]
fn test_brake() {
    let (left, right) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/drive_train/drive/1.0/1.0").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    let status = client.post("/robot/drive_train/brake").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());
}

#[test]
fn test_disable_drive() {
    let (left, right) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/drive_train/drive/1.0/1.0").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    let status = client.post("/robot/drive_train/disable").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    let status = client.post("/robot/drive_train/drive/1.0/1.0").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    let status = client.post("/robot/drive_train/enable").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    let status = client.post("/robot/drive_train/drive/1.0/1.0").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());
}

#[test]
fn test_kill() {
    let (left, right) = create_groups();
    let mut builder = RobotBuilder::new();

    builder.use_custom_drive(left.motor_group, right.motor_group);

    let client = builder.build().launch_tester();

    let status = client.post("/robot/drive_train/drive/1.0/1.0").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    let status = client.post("/robot/kill").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    let status = client.post("/robot/drive_train/drive/1.0/1.0").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    let status = client.post("/robot/revive").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    let status = client.post("/robot/drive_train/drive/1.0/1.0").dispatch().status();
    sleep(Duration::from_millis(TIMEOUT));
    assert_eq!(Status::Ok, status);
    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());
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