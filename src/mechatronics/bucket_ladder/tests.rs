use std::sync::Arc;

use crate::motor_controllers::test_motor::TestMotor;

use super::*;

struct IntakeEnvironment {
    pub state: Arc<GlobalIntakeState>,
    pub life: Arc<GlobalLifeState>,
    pub intake: Intake,
}

fn setup() -> (Arc<GlobalLifeState>, Arc<GlobalIntakeState>, Intake) {
    let state = Arc::new(GlobalIntakeState::new());

    let ladder = Box::new(TestMotor::new(state.get_ladder()));

    let actuator = Box::new(TestMotor::new(state.get_actuator()));

    let life = Arc::new(GlobalLifeState::new());
    let intake = Intake::new(ladder, actuator, state.clone(), life.clone());

    (life, state, intake)
}

#[test]
fn test_setup() {
    let (_, state, _) = setup();

    assert_eq!(0.0, state.get_ladder().get_speed());
    assert_eq!(0.0, state.get_ladder().get_current_state().get_speed());


    assert_eq!(0.0, state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, state.get_actuator().get_speed());
    assert_eq!(false, state.get_enabled());
    assert_eq!(false, state.get_current_state().get_enabled());
}