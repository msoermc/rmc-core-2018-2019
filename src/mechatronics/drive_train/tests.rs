use std::sync::Arc;

use crate::motor_controllers::test_motor::TestMotor;

use super::*;

fn setup() -> (Arc<GlobalLifeState>, Arc<GlobalDriveTrainState>, DriveTrain) {
    let state = Arc::new(GlobalDriveTrainState::new());
    let life = Arc::new(GlobalLifeState::new());
    let left = Box::new(TestMotor::new(state.get_left()));
    let right = Box::new(TestMotor::new(state.get_right()));
    let drive_train = DriveTrain::new(state.clone(), left, right, life.clone());
    (life, state, drive_train)
}

#[test]
fn initial_state() {
    let (_, state, _) = setup();

    assert_eq!(false, state.get_enabled());
    assert_eq!(false, state.get_current_state().get_enabled());
    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_current_state().get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
    assert_eq!(0.0, state.get_current_state().get_right().get_speed());
}

#[test]
fn initial_immobility() {
    let (_, state, mut drive_train) = setup();

    drive_train.drive(1.0, -1.0);

    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
}

#[test]
fn drive_from_stopped() {
    let (_, state, mut drive_train) = setup();

    drive_train.enable();
    drive_train.drive(1.0, -1.0);

    assert_eq!(1.0, state.get_left().get_speed());
    assert_eq!(-1.0, state.get_right().get_speed());
}

#[test]
fn change_direction() {
    let (_, state, mut drive_train) = setup();

    drive_train.enable();
    drive_train.drive(1.0, -1.0);

    drive_train.drive(-1.0, 1.0);
    assert_eq!(-1.0, state.get_left().get_speed());
    assert_eq!(1.0, state.get_right().get_speed());
}

#[test]
fn brake() {
    let (_, state, mut drive_train) = setup();

    drive_train.enable();
    drive_train.drive(1.0, -1.0);

    drive_train.brake();
    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
}

#[test]
fn disable_forced_brake() {
    let (_, state, mut drive_train) = setup();

    drive_train.enable();
    drive_train.drive(1.0, -1.0);

    drive_train.disable();
    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
}

#[test]
fn disable_forced_stasis() {
    let (_, state, mut drive_train) = setup();

    drive_train.disable();
    drive_train.drive(1.0, -1.0);

    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
}

#[test]
fn kill_forced_stasis() {
    let (life, state, mut drive_train) = setup();

    drive_train.enable();
    life.kill();
    drive_train.drive(1.0, -1.0);

    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
}