use std::sync::Arc;

use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::mechatronics::dumper::state::GlobalDumperState;

use super::*;
use crate::devices::motor_controllers::GlobalMotorState;
use crate::devices::motor_controllers::motor_group::MotorGroup;

struct TestMotorGroup {
    pub state: Arc<GlobalMotorState>,
    pub motor_group: Box<MotorController>,
}

fn create_group() -> TestMotorGroup {
    let state = Arc::new(GlobalMotorState::new());

    let test_motor = TestMotor::new(state.clone());

    let test_group = Box::new(test_motor);

    TestMotorGroup { state, motor_group: test_group }
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
    assert_eq!(0.0, group.state.get_speed());

    dumper.run_cycle();

    assert_eq!(false, state.get_enabled());
    assert_eq!(0.0, group.state.get_speed());
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

    assert_eq!(DUMPING_RATE, group.state.get_speed());
    dumper.run_cycle();
    assert_eq!(DUMPING_RATE, group.state.get_speed());

    dumper.disable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.dump();
    assert_eq!(0.0, group.state.get_speed());

    dumper.enable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.dump();
    assert_eq!(DUMPING_RATE, group.state.get_speed());

    life.kill();

    dumper.run_cycle();

    assert_eq!(0.0, group.state.get_speed());
    dumper.dump();
    assert_eq!(0.0, group.state.get_speed());

    dumper.disable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.dump();
    assert_eq!(0.0, group.state.get_speed());

    dumper.enable();
    life.revive();

    dumper.dump();
    assert_eq!(DUMPING_RATE, group.state.get_speed());
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
    assert_eq!(DUMPER_RESET_RATE, group.state.get_speed());
    dumper.run_cycle();
    assert_eq!(DUMPER_RESET_RATE, group.state.get_speed());

    dumper.disable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.reset();
    assert_eq!(0.0, group.state.get_speed());

    dumper.enable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, group.state.get_speed());

    life.kill();

    dumper.run_cycle();

    assert_eq!(0.0, group.state.get_speed());
    dumper.reset();
    assert_eq!(0.0, group.state.get_speed());

    dumper.disable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.reset();
    assert_eq!(0.0, group.state.get_speed());

    dumper.enable();
    life.revive();

    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, group.state.get_speed());
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
    assert_eq!(0.0, group.state.get_speed());
    dumper.run_cycle();
    assert_eq!(0.0, group.state.get_speed());

    dumper.disable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.stop();
    assert_eq!(0.0, group.state.get_speed());

    dumper.enable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.stop();
    assert_eq!(0.0, group.state.get_speed());

    life.kill();
    dumper.run_cycle();

    assert_eq!(0.0, group.state.get_speed());
    dumper.stop();
    assert_eq!(0.0, group.state.get_speed());

    dumper.disable();

    assert_eq!(0.0, group.state.get_speed());
    dumper.stop();
    assert_eq!(0.0, group.state.get_speed());

    dumper.enable();
    life.revive();

    assert_eq!(0.0, group.state.get_speed());
    dumper.stop();
    assert_eq!(0.0, group.state.get_speed());
}