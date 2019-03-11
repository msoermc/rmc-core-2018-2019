use super::*;
use std::sync::Arc;
use crate::motor_controllers::test_motor::TestMotor;

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