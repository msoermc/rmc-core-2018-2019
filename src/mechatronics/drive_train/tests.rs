use super::*;
use std::sync::Arc;
use crate::motor_controllers::test_motor::TestMotor;

#[test]
fn test_setup() {
    let state = Arc::new(GlobalDriveTrainState::new());
    let life = Arc::new(GlobalLifeState::new());
    let left = Box::new(TestMotor::new(state.get_left()));
    let right = Box::new(TestMotor::new(state.get_right()));
    let _drive_train = DriveTrain::new(state.clone(), left, right, life.clone());

    assert_eq!(false, state.get_enabled());
    assert_eq!(false, state.get_current_state().get_enabled());
    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_current_state().get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
    assert_eq!(0.0, state.get_current_state().get_right().get_speed());
}

#[test]
fn test_drive() {
    let state = Arc::new(GlobalDriveTrainState::new());
    let life = Arc::new(GlobalLifeState::new());
    let left = Box::new(TestMotor::new(state.get_left()));
    let right = Box::new(TestMotor::new(state.get_right()));
    let mut drive_train = DriveTrain::new(state.clone(), left, right, life.clone());

    drive_train.enable();
    assert_eq!(true, state.get_enabled());
    assert_eq!(true, state.get_current_state().get_enabled());

    drive_train.drive(1.0, -1.0);
    assert_eq!(1.0, state.get_left().get_speed());
    assert_eq!(1.0, state.get_current_state().get_left().get_speed());
    assert_eq!(-1.0, state.get_right().get_speed());
    assert_eq!(-1.0, state.get_current_state().get_right().get_speed());

    drive_train.disable();
    assert_eq!(false, state.get_enabled());
    assert_eq!(false, state.get_current_state().get_enabled());
    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_current_state().get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
    assert_eq!(0.0, state.get_current_state().get_right().get_speed());
    drive_train.drive(1.0, -1.0);
    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_current_state().get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
    assert_eq!(0.0, state.get_current_state().get_right().get_speed());

    drive_train.enable();
    drive_train.drive(1.0, -1.0);
    life.kill();
    drive_train.run_cycle();
    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_current_state().get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
    assert_eq!(0.0, state.get_current_state().get_right().get_speed());

    life.revive();
    drive_train.drive(1.0, -1.0);
    drive_train.disable();
    drive_train.run_cycle();
    assert_eq!(false, state.get_enabled());
    assert_eq!(false, state.get_current_state().get_enabled());
    assert_eq!(0.0, state.get_left().get_speed());
    assert_eq!(0.0, state.get_current_state().get_left().get_speed());
    assert_eq!(0.0, state.get_right().get_speed());
    assert_eq!(0.0, state.get_current_state().get_right().get_speed());
}
