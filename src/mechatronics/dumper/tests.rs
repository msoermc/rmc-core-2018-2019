use super::*;
use std::sync::Arc;
use crate::motor_controllers::test_motor::TestMotor;

#[test]
fn test_setup() {
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());
    let motor = Box::new(TestMotor::new(state.get_motor()));

    let _dumper = Dumper::new(life.clone(), motor, state.clone());

    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(false, state.get_enabled());
}

#[test]
fn test_dump() {
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());
    let motor = Box::new(TestMotor::new(state.get_motor()));

    let mut dumper = Dumper::new(life.clone(), motor, state.clone());
    dumper.enable();

    dumper.dump();
    assert_eq!(DUMPING_RATE, state.get_motor().get_speed());
    dumper.run_cycle();
    assert_eq!(DUMPING_RATE, state.get_motor().get_speed());

    life.kill();
    dumper.run_cycle();
    assert_eq!(0.0, state.get_motor().get_speed());
    dumper.dump();
    assert_eq!(0.0, state.get_motor().get_speed());
    state.get_motor().set_speed(1.0);
    dumper.run_cycle();
    assert_eq!(0.0, state.get_motor().get_speed());

    life.revive();
    dumper.dump();
    assert_eq!(DUMPING_RATE, state.get_motor().get_speed());

    dumper.disable();
    assert_eq!(0.0, state.get_motor().get_speed());
    dumper.dump();
    assert_eq!(0.0, state.get_motor().get_speed());
    state.get_motor().set_speed(1.0);
    dumper.run_cycle();
    assert_eq!(0.0, state.get_motor().get_speed());

    dumper.enable();
    dumper.dump();
    assert_eq!(DUMPING_RATE, state.get_motor().get_speed());
}

#[test]
fn test_reset() {
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());
    let motor = Box::new(TestMotor::new(state.get_motor()));

    let mut dumper = Dumper::new(life.clone(), motor, state.clone());
    dumper.enable();

    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
    assert_eq!(DUMPER_RESET_RATE, state.get_current_state().get_motor().get_speed());
    dumper.run_cycle();
    assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
    assert_eq!(DUMPER_RESET_RATE, state.get_current_state().get_motor().get_speed());

    life.kill();
    dumper.run_cycle();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(0.0, state.get_current_state().get_motor().get_speed());

    dumper.reset();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(0.0, state.get_current_state().get_motor().get_speed());
    state.get_motor().set_speed(1.0);
    dumper.run_cycle();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(0.0, state.get_current_state().get_motor().get_speed());

    life.revive();
    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
    assert_eq!(DUMPER_RESET_RATE, state.get_current_state().get_motor().get_speed());

    dumper.disable();
    assert_eq!(false, state.get_enabled());
    assert_eq!(false, state.get_current_state().get_enabled());
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(0.0, state.get_current_state().get_motor().get_speed());

    dumper.reset();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(0.0, state.get_current_state().get_motor().get_speed());
    state.get_motor().set_speed(1.0);
    dumper.run_cycle();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(0.0, state.get_current_state().get_motor().get_speed());

    dumper.enable();
    assert_eq!(true, state.get_enabled());
    assert_eq!(true, state.get_current_state().get_enabled());
    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
    assert_eq!(DUMPER_RESET_RATE, state.get_current_state().get_motor().get_speed());
}
