use super::*;
use std::sync::Arc;
use crate::motor_controllers::test_motor::TestMotor;

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
fn test_dump() {
    let (_, state, mut dumper) = setup();

    dumper.enable();

    dumper.dump();
    assert_eq!(0.0, state.get_motor().get_speed());
}