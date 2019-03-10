use std::sync::Arc;

use crate::motor_controllers::test_motor::TestMotor;

use super::*;

struct IntakeEnvironment {
    pub state: Arc<GlobalIntakeState>,
    pub life: Arc<GlobalLifeState>,
    pub intake: Intake,
}

fn create_environment() -> IntakeEnvironment {
    let state = Arc::new(GlobalIntakeState::new());

    let ladder = Box::new(TestMotor::new(state.get_ladder()));

    let actuator = Box::new(TestMotor::new(state.get_actuator()));

    let life = Arc::new(GlobalLifeState::new());
    let intake = Intake::new(ladder, actuator, state.clone(), life.clone());

    IntakeEnvironment {
        state,
        life,
        intake,
    }
}

#[test]
fn test_setup() {
    let environment = create_environment();

    assert_eq!(0.0, environment.state.get_ladder().get_speed());
    assert_eq!(0.0, environment.state.get_ladder().get_current_state().get_speed());


    assert_eq!(0.0, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_actuator().get_speed());
    assert_eq!(false, environment.state.get_enabled());
    assert_eq!(false, environment.state.get_current_state().get_enabled());
}

#[test]
fn test_dig_actuators() {
    let mut environment = create_environment();
    environment.intake.enable();

    environment.intake.dig();
    assert_eq!(DIGGING_RATE, environment.state.get_current_state().get_digger().get_speed());
    assert_eq!(DIGGING_RATE, environment.state.get_ladder().get_current_state().get_speed());
    assert_eq!(DIGGING_RATE, environment.state.get_ladder().get_speed());

    environment.intake.run_cycle();
    assert_eq!(DIGGING_RATE, environment.state.get_current_state().get_digger().get_speed());
    assert_eq!(DIGGING_RATE, environment.state.get_ladder().get_current_state().get_speed());
    assert_eq!(DIGGING_RATE, environment.state.get_ladder().get_speed());

    environment.intake.disable();
    environment.intake.dig();
    assert_eq!(0.0, environment.state.get_current_state().get_digger().get_speed());
    assert_eq!(0.0, environment.state.get_ladder().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_ladder().get_speed());
    environment.intake.run_cycle();
    assert_eq!(0.0, environment.state.get_current_state().get_digger().get_speed());
    assert_eq!(0.0, environment.state.get_ladder().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_ladder().get_speed());
}

#[test]
fn test_stop_digger() {
    let mut environment = create_environment();
    environment.intake.enable();
    environment.intake.dig();
    environment.intake.stop_ladder();
    assert_eq!(0.0, environment.state.get_ladder().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_ladder().get_speed());
}

#[test]
fn test_raise_actuators() {
    let mut environment = create_environment();
    environment.intake.enable();

    environment.intake.raise();

    assert_eq!(MH_ACTUATOR_RATE, environment.state.get_current_state().get_actuator().get_speed());


    assert_eq!(MH_ACTUATOR_RATE, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(MH_ACTUATOR_RATE, environment.state.get_actuator().get_speed());

    environment.intake.run_cycle();


    assert_eq!(MH_ACTUATOR_RATE, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(MH_ACTUATOR_RATE, environment.state.get_actuator().get_speed());

    environment.intake.disable();
    environment.intake.raise();

    assert_eq!(0.0, environment.state.get_current_state().get_actuator().get_speed());


    assert_eq!(0.0, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_actuator().get_speed());
    environment.intake.run_cycle();


    assert_eq!(0.0, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_actuator().get_speed());
}

#[test]
fn test_lower_actuators() {
    let mut environment = create_environment();
    environment.intake.enable();

    environment.intake.lower();


    assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_actuator().get_speed());

    environment.intake.run_cycle();


    assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_actuator().get_speed());

    environment.intake.disable();
    environment.intake.lower();

    assert_eq!(0.0, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_actuator().get_speed());
    environment.intake.run_cycle();


    assert_eq!(0.0, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_actuator().get_speed());
}

#[test]
fn test_raise_limits() {
    let mut environment = create_environment();
    environment.intake.enable();

    environment.state.get_right_actuator().set_upper(true);
    environment.state.get_left_actuator().set_upper(true);

    assert_eq!(true, environment.state.get_right_actuator().get_current_state().get_upper());
    assert_eq!(true, environment.state.get_left_actuator().get_current_state().get_upper());

    assert_eq!(true, environment.state.get_current_state().get_right_actuator().get_upper());
    assert_eq!(true, environment.state.get_current_state().get_left_actuator().get_upper());

    environment.intake.raise();

    assert_eq!(0.0, environment.state.get_current_state().get_actuator().get_speed());


    assert_eq!(0.0, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_actuator().get_speed());

    environment.intake.lower();


    assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_actuator().get_speed());
}

#[test]
fn test_lower_limits() {
    let mut environment = create_environment();
    environment.intake.enable();

    environment.state.get_right_actuator().set_lower(true);
    environment.state.get_left_actuator().set_lower(true);

    assert_eq!(true, environment.state.get_right_actuator().get_current_state().get_lower());
    assert_eq!(true, environment.state.get_left_actuator().get_current_state().get_lower());

    assert_eq!(true, environment.state.get_current_state().get_right_actuator().get_lower());
    assert_eq!(true, environment.state.get_current_state().get_left_actuator().get_lower());

    environment.intake.lower();

    assert_eq!(0.0, environment.state.get_current_state().get_actuator().get_speed());

    assert_eq!(0.0, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_actuator().get_speed());

    environment.intake.raise();

    assert_eq!(MH_ACTUATOR_RATE, environment.state.get_current_state().get_actuator().get_speed());


    assert_eq!(MH_ACTUATOR_RATE, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(MH_ACTUATOR_RATE, environment.state.get_actuator().get_speed());
}

#[test]
fn test_stop_actuators() {
    let mut environment = create_environment();
    environment.intake.enable();
    environment.intake.raise();
    environment.intake.stop_actuators();


    assert_eq!(0.0, environment.state.get_actuator().get_current_state().get_speed());
    assert_eq!(0.0, environment.state.get_actuator().get_speed());
}

#[test]
fn test_enabling() {
    let mut environment = create_environment();
    environment.intake.enable();
    assert_eq!(true, environment.state.get_enabled());
    assert_eq!(true, environment.state.get_current_state().get_enabled());
    environment.intake.disable();
    assert_eq!(false, environment.state.get_enabled());
    assert_eq!(false, environment.state.get_current_state().get_enabled());
}
