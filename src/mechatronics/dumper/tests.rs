use std::sync::Arc;

use crate::motor_controllers::test_motor::TestMotor;

use super::*;

fn setup() -> (Arc<GlobalLifeState>, Arc<GlobalDumperState>, Dumper) {
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());
    let motor = Box::new(TestMotor::new(state.get_motor()));

    let dumper = Dumper::new(life.clone(), motor, state.clone());

    (life, state, dumper)
}

#[test]
fn initial_state() {
    let (_, state, _) = setup();

    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(false, state.get_enabled());
}

#[test]
fn initial_immobility() {
    let (_, state, mut dumper) = setup();

    dumper.dump();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(false, state.get_enabled());

    dumper.reset();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(false, state.get_enabled());
}

#[test]
fn dump() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    dumper.dump();
    assert_eq!(DUMPING_RATE, state.get_motor().get_speed());
}

#[test]
fn reset() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    dumper.reset();
    assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
}

#[test]
fn stop() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    dumper.dump();
    dumper.stop();
    assert_eq!(0.0, state.get_motor().get_speed());
}

#[test]
fn disable_stasis() {
    let (_, state, mut dumper) = setup();

    dumper.disable();

    dumper.dump();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(false, state.get_enabled());

    dumper.reset();
    assert_eq!(0.0, state.get_motor().get_speed());
    assert_eq!(false, state.get_enabled());
}

#[test]
fn disable_stop() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    dumper.dump();
    dumper.disable();
    assert_eq!(0.0, state.get_motor().get_speed());
}

#[test]
fn enable() {
    let (_, state, mut dumper) = setup();

    dumper.disable();
    dumper.enable();

    dumper.dump();
    assert_eq!(DUMPING_RATE, state.get_motor().get_speed());
}

#[test]
fn kill_stasis() {
    let (life, state, mut dumper) = setup();

    life.kill();
    dumper.enable();

    dumper.dump();
    assert_eq!(0.0, state.get_motor().get_speed());

    dumper.reset();
    assert_eq!(0.0, state.get_motor().get_speed());
}

#[test]
fn upper_limit_stop() {
    let (_, state, mut dumper) = setup();

    dumper.enable();
    dumper.dump();

    state.get_upper_limit().store(true, Ordering::SeqCst);
    dumper.run_cycle();

    assert_eq!(0.0, state.get_motor().get_speed());
}

#[test]
fn upper_limit_stasis() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    state.get_upper_limit().store(true, Ordering::SeqCst);
    dumper.dump();

    assert_eq!(0.0, state.get_motor().get_speed());
}

#[test]
fn upper_limit_reverse() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    state.get_upper_limit().store(true, Ordering::SeqCst);
    dumper.reset();

    assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
}

#[test]
fn test_upper_limit_maintain_reset() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    dumper.reset();
    state.get_upper_limit().store(true, Ordering::SeqCst);

    assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
}

#[test]
fn lower_limit_stop() {
    let (_, state, mut dumper) = setup();

    dumper.enable();
    dumper.reset();

    state.get_lower_limit().store(true, Ordering::SeqCst);
    dumper.run_cycle();

    assert_eq!(0.0, state.get_motor().get_speed());
}

#[test]
fn lower_limit_stasis() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    state.get_lower_limit().store(true, Ordering::SeqCst);
    dumper.reset();

    assert_eq!(0.0, state.get_motor().get_speed());
}

#[test]
fn lower_limit_reverse() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    state.get_lower_limit().store(true, Ordering::SeqCst);
    dumper.dump();

    assert_eq!(DUMPING_RATE, state.get_motor().get_speed());
}

#[test]
fn test_lower_limit_maintain_dump() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    dumper.dump();
    state.get_lower_limit().store(true, Ordering::SeqCst);

    assert_eq!(DUMPING_RATE, state.get_motor().get_speed());
}