use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::robot_map::*;

use super::*;
use crate::mechatronics::dumper::state::GlobalDumperState;

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
fn test_setup() {
    let group = create_group();
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());

    let motor = group.motor_group;

    let mut dumper = Dumper::new(life.clone(), motor, state.clone());

    life.revive();

    assert_eq!(false, state.get_enabled());
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.run_cycle();

    assert_eq!(false, state.get_enabled());
    assert_eq!(0.0, *group.speed.read().unwrap());
}

#[test]
fn test_enabled() {
    let group = create_group();
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());

    let motor = group.motor_group;

    let mut dumper = Dumper::new(life.clone(), motor, state.clone());

    life.revive();

    dumper.enable();

    assert_eq!(true, state.get_enabled());

    dumper.disable();

    assert_eq!(false, state.get_enabled());
}

#[test]
fn test_dumping() {
    let group = create_group();
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());

    let motor = group.motor_group;

    let mut dumper = Dumper::new(life.clone(), motor, state.clone());

    state.set_enabled(true);
    life.revive();

    dumper.dump();

    assert_eq!(DUMPING_RATE, *group.speed.read().unwrap());
    dumper.run_cycle();
    assert_eq!(DUMPING_RATE, *group.speed.read().unwrap());

    dumper.disable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.dump();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.enable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.dump();
    assert_eq!(DUMPING_RATE, *group.speed.read().unwrap());

    life.kill();

    dumper.run_cycle();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.dump();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.disable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.dump();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.enable();
    life.revive();

    dumper.dump();
    assert_eq!(DUMPING_RATE, *group.speed.read().unwrap());
}

#[test]
fn test_reset() {
    let group = create_group();
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());

    let motor = group.motor_group;

    let mut dumper = Dumper::new(life.clone(), motor, state.clone());

    state.set_enabled(true);
    life.revive();

    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, *group.speed.read().unwrap());
    dumper.run_cycle();
    assert_eq!(DUMPER_RESET_RATE, *group.speed.read().unwrap());

    dumper.disable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.reset();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.enable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, *group.speed.read().unwrap());

    life.kill();

    dumper.run_cycle();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.reset();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.disable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.reset();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.enable();
    life.revive();

    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, *group.speed.read().unwrap());
}

#[test]
fn test_stop() {
    let group = create_group();
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());

    let motor = group.motor_group;

    let mut dumper = Dumper::new(life.clone(), motor, state.clone());

    state.set_enabled(true);
    life.revive();

    dumper.stop();
    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.run_cycle();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.disable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.stop();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.enable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.stop();
    assert_eq!(0.0, *group.speed.read().unwrap());

    life.kill();
    dumper.run_cycle();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.stop();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.disable();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.stop();
    assert_eq!(0.0, *group.speed.read().unwrap());

    dumper.enable();
    life.revive();

    assert_eq!(0.0, *group.speed.read().unwrap());
    dumper.stop();
    assert_eq!(0.0, *group.speed.read().unwrap());
}