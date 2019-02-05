use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::MotorStateKind;
use crate::devices::motor_controllers::test_motor::TestMotor;

use super::*;

struct TestMotorGroup {
    pub inverted: Arc<RwLock<bool>>,
    pub speed: Arc<RwLock<f32>>,
    pub motor_group: MotorGroup,
}

#[test]
fn test_cycle_no_fail_no_inversion() {
    let (left, right) = create_groups();
    let status = GlobalLifeStatus::new();

    let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status.clone());

    // Make sure we are setup correctly
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test both forwards
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Test cycle
    drive_train.run_cycle();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Test brake
    drive_train.brake();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test cycle
    drive_train.run_cycle();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test both forwards
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Kill
    status.kill();

    // Test cycle
    drive_train.run_cycle();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Revive
    status.revive();

    // Test cycle
    drive_train.drive(1.0, 1.0);
    drive_train.run_cycle();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Disable
    drive_train.disable();

    // Test cycle
    drive_train.run_cycle();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Enable
    drive_train.enable();

    // Test cycle
    drive_train.drive(1.0, 1.0);
    drive_train.run_cycle();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());
}

#[test]
fn test_drive_no_fail_no_inversion() {
    let (left, right) = create_groups();
    let status = GlobalLifeStatus::new();

    let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status);

    // Make sure we are setup correctly
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test both forwards
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Test both backwards
    drive_train.drive(-1.0, -1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(-1.0, *left.speed.read().unwrap());
    assert_eq!(-1.0, *right.speed.read().unwrap());

    // Test right forwards and left backwards
    drive_train.drive(-1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(-1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Test right backwards and left forwards
    drive_train.drive(1.0, -1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(-1.0, *right.speed.read().unwrap());
}

#[test]
fn test_brake_no_fail_no_inversion() {
    let (left, right) = create_groups();
    let status = GlobalLifeStatus::new();

    let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status);

    // Make sure we are setup correctly
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test both forwards
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Test brake
    drive_train.brake();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());
}

#[test]
fn test_enabling_no_fail_no_inversion() {
    let (left, right) = create_groups();
    let status = GlobalLifeStatus::new();

    let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status);

    // Make sure we are setup correctly
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test both forwards
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Test disable
    drive_train.disable();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Make sure we can't still drive
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test enable
    drive_train.enable();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Make sure we can drive
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());
}

#[test]
fn test_killing_no_fail_no_inversion() {
    let (left, right) = create_groups();
    let status = GlobalLifeStatus::new();

    let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status.clone());

    // Make sure we are setup correctly
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test both forwards
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());

    // Test kill
    status.kill();
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Test revive
    status.revive();
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(0.0, *left.speed.read().unwrap());
    assert_eq!(0.0, *right.speed.read().unwrap());

    // Make sure we can drive
    drive_train.drive(1.0, 1.0);
    assert_eq!(false, *left.inverted.read().unwrap());
    assert_eq!(false, *right.inverted.read().unwrap());

    assert_eq!(1.0, *left.speed.read().unwrap());
    assert_eq!(1.0, *right.speed.read().unwrap());
}

#[test]
fn test_get_state_no_fail() {
    let (left, right) = create_groups();
    let status = GlobalLifeStatus::new();

    let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status.clone());

    drive_train.drive(1.0, 1.0);

    assert!(drive_train.get_motor_states().iter().map(|status| status.get_kind() == MotorStateKind::Ok).fold(true, |old, new| old && new));
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
