use std::sync::Arc;

use crate::motor_controllers::test_motor::TestMotor;

use super::*;

fn setup() -> (Arc<GlobalLifeState>, Arc<GlobalIntakeState>, Intake) {
    let state = Arc::new(GlobalIntakeState::new());

    let ladder = Box::new(TestMotor::new(state.get_digger()));
    let actuator = Box::new(TestMotor::new(state.get_actuator()));

    let life = Arc::new(GlobalLifeState::new());
    let intake = Intake::new(ladder, actuator, state.clone(), life.clone());

    (life, state, intake)
}

#[test]
fn initial_state() {
    let (_, state, _) = setup();

    assert_eq!(0.0, state.get_digger().get_speed());

    assert_eq!(0.0, state.get_actuator().get_speed());
    assert_eq!(false, state.get_enabled());
}

#[test]
fn initial_digger_immobility() {
    let (_, state, mut intake) = setup();

    intake.dig();
    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn initial_actuator_immobility() {
    let (_, state, mut intake) = setup();

    intake.raise();
    assert_eq!(0.0, state.get_actuator().get_speed());

    intake.lower();
    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn digging() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.dig();
    assert_eq!(DIGGING_RATE, state.get_digger().get_speed());
}

#[test]
fn reversing() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.reverse();
    assert_eq!(-DIGGING_RATE, state.get_digger().get_speed());
}

#[test]
fn stop_digging() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.dig();
    intake.stop_digging();
    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn stop_reverse() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.dig();
    intake.stop_digging();
    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn raise() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.raise();
    assert_eq!(MH_ACTUATOR_RATE, state.get_actuator().get_speed());
}

#[test]
fn lower() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.lower();
    assert_eq!(-MH_ACTUATOR_RATE, state.get_actuator().get_speed());
}

#[test]
fn disable_stop_digger() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.dig();
    intake.disable();

    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn disable_stop_reverse() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.reverse();
    intake.disable();

    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn disable_stop_actuator() {
    let (_, state, mut intake) = setup();
    intake.enable();

    intake.raise();
    intake.disable();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn disable_digger_stasis() {
    let (_, state, mut intake) = setup();

    intake.disable();

    intake.dig();
    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn disable_digger_reverse_stasis() {
    let (_, state, mut intake) = setup();

    intake.disable();

    intake.reverse();
    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn disable_actuator_stasis() {
    let (_, state, mut intake) = setup();

    intake.disable();

    intake.raise();
    assert_eq!(0.0, state.get_actuator().get_speed());

    intake.lower();
    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn kill_actuator_raise_stasis() {
    let (life, state, mut intake) = setup();
    intake.enable();

    life.kill();

    intake.raise();
    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn kill_actuator_lower_stasis() {
    let (life, state, mut intake) = setup();
    intake.enable();

    life.kill();

    intake.lower();
    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn kill_digger_stasis() {
    let (life, state, mut intake) = setup();

    life.kill();

    intake.dig();
    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn kill_digger_reverse_stasis() {
    let (life, state, mut intake) = setup();

    life.kill();

    intake.reverse();
    assert_eq!(0.0, state.get_digger().get_speed());
}

#[test]
fn upper_left_limit_stop() {
    let (_, state, mut intake) = setup();
    intake.enable();
    intake.raise();

    state.get_left_actuator().get_upper().store(true, Ordering::SeqCst);
    intake.run_cycle();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn lower_left_limit_stop() {
    let (_, state, mut intake) = setup();
    intake.enable();
    intake.lower();

    state.get_left_actuator().get_lower().store(true, Ordering::SeqCst);
    intake.run_cycle();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn upper_right_limit_stop() {
    let (_, state, mut intake) = setup();
    intake.enable();
    intake.raise();

    state.get_right_actuator().get_upper().store(true, Ordering::SeqCst);
    intake.run_cycle();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn lower_right_limit_stop() {
    let (_, state, mut intake) = setup();
    intake.enable();
    intake.lower();

    state.get_right_actuator().get_lower().store(true, Ordering::SeqCst);
    intake.run_cycle();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn upper_both_limit_stop() {
    let (_, state, mut intake) = setup();
    intake.enable();
    intake.raise();

    state.get_left_actuator().set_upper(true);
    state.get_right_actuator().set_upper(true);
    intake.run_cycle();

    assert_eq!(0.0, state.get_actuator().get_speed());
    assert_eq!(true, state.get_left_actuator().get_upper().load(Ordering::SeqCst));
    assert_eq!(true, state.get_right_actuator().get_upper().load(Ordering::SeqCst));
    assert_eq!(true, state.get_current_state().get_left_actuator().get_upper());
    assert_eq!(true, state.get_current_state().get_right_actuator().get_upper());
}

#[test]
fn lower_both_limit_stop() {
    let (_, state, mut intake) = setup();
    intake.enable();
    intake.lower();

    state.get_right_actuator().set_lower(true);
    state.get_left_actuator().set_lower(true);
    intake.run_cycle();

    assert_eq!(0.0, state.get_actuator().get_speed());
    assert_eq!(true, state.get_left_actuator().get_lower().load(Ordering::SeqCst));
    assert_eq!(true, state.get_right_actuator().get_lower().load(Ordering::SeqCst));
    assert_eq!(true, state.get_current_state().get_left_actuator().get_lower());
    assert_eq!(true, state.get_current_state().get_right_actuator().get_lower());
}

#[test]
fn upper_left_limit_stasis() {
    let (_, state, mut intake) = setup();
    intake.enable();

    state.get_left_actuator().get_upper().store(true, Ordering::SeqCst);
    intake.raise();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn lower_left_limit_stasis() {
    let (_, state, mut intake) = setup();
    intake.enable();

    state.get_left_actuator().get_lower().store(true, Ordering::SeqCst);
    intake.lower();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn upper_right_limit_stasis() {
    let (_, state, mut intake) = setup();
    intake.enable();

    state.get_right_actuator().get_upper().store(true, Ordering::SeqCst);
    intake.raise();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn lower_right_limit_stasis() {
    let (_, state, mut intake) = setup();
    intake.enable();

    state.get_right_actuator().get_lower().store(true, Ordering::SeqCst);
    intake.lower();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn upper_both_limit_stasis() {
    let (_, state, mut intake) = setup();
    intake.enable();

    state.get_left_actuator().get_upper().store(true, Ordering::SeqCst);
    state.get_right_actuator().get_upper().store(true, Ordering::SeqCst);
    intake.raise();

    assert_eq!(0.0, state.get_actuator().get_speed());
}

#[test]
fn lower_both_limit_stasis() {
    let (_, state, mut intake) = setup();
    intake.enable();

    state.get_left_actuator().get_lower().store(true, Ordering::SeqCst);
    state.get_right_actuator().get_lower().store(true, Ordering::SeqCst);
    intake.lower();

    assert_eq!(0.0, state.get_actuator().get_speed());
}