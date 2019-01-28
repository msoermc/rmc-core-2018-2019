use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::robot_map::*;

use super::*;

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
    TestMotorGroup { inverted, speed, motor_group: test_group }
}

#[test]
fn test_construction() {
    let motors = create_group();
    let life = GlobalLifeStatus::new();
    let dumper = Dumper::new(life, motors.motor_group);

    assert_eq!(0.0, *motors.speed.read().unwrap());
}

#[test]
fn test_dumping() {
    let motors = create_group();
    let life = GlobalLifeStatus::new();
    let mut dumper = Dumper::new(life, motors.motor_group);

    dumper.dump();
    assert_eq!(DUMPING_RATE, *motors.speed.read().unwrap());
    dumper.run_cycle();
    assert_eq!(DUMPING_RATE, *motors.speed.read().unwrap());
}

#[test]
fn test_resetting() {
    let motors = create_group();
    let life = GlobalLifeStatus::new();
    let mut dumper = Dumper::new(life, motors.motor_group);

    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, *motors.speed.read().unwrap());
    dumper.run_cycle();
    assert_eq!(DUMPER_RESET_RATE, *motors.speed.read().unwrap());
}

#[test]
fn test_stopping() {
    let motors = create_group();
    let life = GlobalLifeStatus::new();
    let mut dumper = Dumper::new(life, motors.motor_group);

    dumper.dump();
    dumper.stop();
    assert_eq!(0.0, *motors.speed.read().unwrap());
}