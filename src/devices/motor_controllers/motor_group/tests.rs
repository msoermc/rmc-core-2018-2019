use super::*;
use std::sync::Arc;
use std::sync::RwLock;
use crate::devices::motor_controllers::test_motor::TestMotor;

struct TestMotorGroup {
    pub inverted: Arc<RwLock<bool>>,
    pub speed: Arc<RwLock<f32>>,
    pub motor_group: MotorGroup,
}

fn create_group() -> TestMotorGroup {
    let inverted = Arc::new(RwLock::new(false));

    let speed = Arc::new(RwLock::new(0.0));

    let test_motor = TestMotor::new(inverted.clone(), speed.clone());

    let test_group = MotorGroup::new(vec![Box::new(test_motor)]);

    TestMotorGroup {inverted, speed, motor_group: test_group}
}

#[test]
fn test_set_speed_no_fail_no_inversion() {
    let mut group = create_group();

    // Test setup
    assert_eq!(0.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());

    // Go forwards
    group.motor_group.set_speed(1.0).expect("Command should not have failed!");
    assert_eq!(1.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());

    // Go backwards
    group.motor_group.set_speed(-1.0).expect("Command should not have failed!");
    assert_eq!(-1.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());
}

#[test]
fn test_stop_no_fail_no_inversion() {
    let mut group = create_group();

    // Test setup
    assert_eq!(0.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());

    // Go forwards
    group.motor_group.set_speed(1.0).expect("Command should not have failed!");
    assert_eq!(1.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());

    // Stop
    group.motor_group.stop().expect("Command should not have failed!");
    assert_eq!(0.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());
}

#[test]
fn test_inversion() {
    let mut group = create_group();

    // Test setup
    assert_eq!(0.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());

    // Invert
    group.motor_group.invert().unwrap();

    // Go forwards
    group.motor_group.set_speed(1.0).expect("Command should not have failed!");
    assert_eq!(-1.0, *group.speed.read().unwrap());
    assert_eq!(true, *group.inverted.read().unwrap());

    // Stop
    group.motor_group.stop().expect("Command should not have failed!");
    assert_eq!(0.0, *group.speed.read().unwrap());
    assert_eq!(true, *group.inverted.read().unwrap());

    // Go backwards
    group.motor_group.set_speed(-1.0).expect("Command should not have failed!");
    assert_eq!(1.0, *group.speed.read().unwrap());
    assert_eq!(true, *group.inverted.read().unwrap());

    // Invert
    group.motor_group.invert().unwrap();

    // Go forwards
    group.motor_group.set_speed(1.0).expect("Command should not have failed!");
    assert_eq!(1.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());

    // Stop
    group.motor_group.stop().expect("Command should not have failed!");
    assert_eq!(0.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());

    // Go backwards
    group.motor_group.set_speed(-1.0).expect("Command should not have failed!");
    assert_eq!(-1.0, *group.speed.read().unwrap());
    assert_eq!(false, *group.inverted.read().unwrap());
}
