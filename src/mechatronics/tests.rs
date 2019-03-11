use std::sync::Arc;
use std::sync::mpsc::sync_channel;

use crate::framework::Runnable;
use crate::mechatronics::bucket_ladder::Intake;
use crate::mechatronics::commands::RobotCommandFactory;
use crate::mechatronics::controller::RobotController;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::dumper::Dumper;
use crate::motor_controllers::test_motor::TestMotor;
use crate::status::life::GlobalLifeState;
use crate::status::robot_state::GlobalRobotState;

use super::*;
use crate::robot_map::{DIGGING_RATE, MH_ACTUATOR_RATE, DUMPING_RATE, DUMPER_RESET_RATE};

fn setup() -> (Arc<GlobalRobotState>, RobotController, RobotCommandFactory) {
    let state = Arc::new(GlobalRobotState::new());

    let digger = Box::new(TestMotor::new(state.get_intake().get_digger()));
    let intake_height = Box::new(TestMotor::new(state.get_intake().get_actuator()));
    let intake = Intake::new(digger, intake_height, state.get_intake(), state.get_life());

    let dumper_motor = Box::new(TestMotor::new(state.get_dumper().get_motor()));
    let dumper = Dumper::new(state.get_life(), dumper_motor, state.get_dumper());

    let left_drive = Box::new(TestMotor::new(state.get_drive().get_left()));
    let right_drive = Box::new(TestMotor::new(state.get_drive().get_right()));
    let drive_train = DriveTrain::new(state.get_drive(), left_drive, right_drive, state.get_life());

    let (sender, receiver) = sync_channel(10);

//    let messenger = RobotMessenger::new(sender);
    let controller = RobotController::new(receiver, drive_train, dumper, intake, state.get_life(), state.get_cycle_counter());

    (state, controller, RobotCommandFactory::new())
}

#[test]
fn kill_status() {
    let (state, mut controller, factory) = setup();
    controller.handle_message(Box::new(factory.generate_kill_command()));

    assert_eq!(false, state.get_life().is_alive());
}

#[test]
fn kill_drive() {
    let (state, mut controller, factory) = setup();

    controller.get_drive_train().enable();
    controller.get_drive_train().drive(1.0, 1.0);

    controller.handle_message(Box::new(factory.generate_kill_command()));

    assert_eq!(true, state.get_drive().get_enabled());
    assert_eq!(0.0, state.get_drive().get_left().get_speed());
    assert_eq!(0.0, state.get_drive().get_right().get_speed());
}

#[test]
fn kill_dumper() {
    let (state, mut controller, factory) = setup();

    controller.get_dumper().enable();
    controller.get_dumper().dump();

    controller.handle_message(Box::new(factory.generate_kill_command()));

    assert_eq!(true, state.get_dumper().get_enabled());
    assert_eq!(0.0, state.get_dumper().get_motor().get_speed());
}

#[test]
fn kill_intake() {
    let (state, mut controller, factory) = setup();

    controller.get_intake().enable();
    controller.get_intake().dig();
    controller.get_intake().raise();

    controller.handle_message(Box::new(factory.generate_kill_command()));

    assert_eq!(true, state.get_intake().get_enabled());
    assert_eq!(0.0, state.get_intake().get_digger().get_speed());
    assert_eq!(0.0, state.get_intake().get_actuator().get_speed());
}

#[test]
fn drive() {
    let (state, mut controller, factory) = setup();

    controller.get_drive_train().enable();

    controller.handle_message(Box::new(factory.generate_drive_command(1.0, -1.0).unwrap()));

    assert_eq!(1.0, state.get_drive().get_left().get_speed());
    assert_eq!(-1.0, state.get_drive().get_right().get_speed());
}

#[test]
fn invalid_drive_values() {
    let (_, _, factory) = setup();

    assert!(factory.generate_drive_command(2.0, 1.0).is_none());
    assert!(factory.generate_drive_command(-2.0, 1.0).is_none());
    assert!(factory.generate_drive_command(1.0, 2.0).is_none());
    assert!(factory.generate_drive_command(1.0, -2.0).is_none());
    assert!(factory.generate_drive_command(-2.0, -2.0).is_none());
    assert!(factory.generate_drive_command(2.0, 2.0).is_none());
    assert!(factory.generate_drive_command(-2.0, 2.0).is_none());
    assert!(factory.generate_drive_command(2.0, -2.0).is_none());
}

#[test]
fn brake() {
    let (state, mut controller, factory) = setup();

    controller.get_drive_train().enable();

    controller.handle_message(Box::new(factory.generate_drive_command(1.0, -1.0).unwrap()));
    controller.handle_message(Box::new(factory.generate_brake_command()));

    assert_eq!(0.0, state.get_drive().get_left().get_speed());
    assert_eq!(0.0, state.get_drive().get_right().get_speed());
}

#[test]
fn dig() {
    let (state, mut controller, factory) = setup();

    controller.get_intake().enable();

    controller.handle_message(Box::new(factory.generate_dig_command()));

    assert_eq!(DIGGING_RATE, state.get_intake().get_digger().get_speed());
}

#[test]
fn stop_digging() {
    let (state, mut controller, factory) = setup();

    controller.get_intake().enable();
    controller.get_intake().dig();

    controller.handle_message(Box::new(factory.generate_stop_digger_command()));

    assert_eq!(0.0, state.get_intake().get_digger().get_speed());
}

#[test]
fn raise() {
    let (state, mut controller, factory) = setup();

    controller.get_intake().enable();

    controller.handle_message(Box::new(factory.generate_raise_actuators_command()));

    assert_eq!(MH_ACTUATOR_RATE, state.get_intake().get_actuator().get_speed());
}

#[test]
fn lower() {
    let (state, mut controller, factory) = setup();

    controller.get_intake().enable();

    controller.handle_message(Box::new(factory.generate_lower_actuators_command()));

    assert_eq!(-MH_ACTUATOR_RATE, state.get_intake().get_actuator().get_speed());
}

#[test]
fn stop_rising() {
    let (state, mut controller, factory) = setup();

    controller.get_intake().enable();
    controller.get_intake().raise();

    controller.handle_message(Box::new(factory.generate_stop_actuators_command()));

    assert_eq!(0.0, state.get_intake().get_actuator().get_speed());
}

#[test]
fn dump() {
    let (state, mut controller, factory) = setup();

    controller.get_dumper().enable();

    controller.handle_message(Box::new(factory.generate_dump_command()));

    assert_eq!(DUMPING_RATE, state.get_dumper().get_motor().get_speed());
}

#[test]
fn reset_dumper() {
    let (state, mut controller, factory) = setup();

    controller.get_dumper().enable();

    controller.handle_message(Box::new(factory.generate_reset_dumper_command()));

    assert_eq!(DUMPER_RESET_RATE, state.get_dumper().get_motor().get_speed());
}

#[test]
fn stop_dumper() {
    let (state, mut controller, factory) = setup();

    controller.get_dumper().enable();
    controller.get_dumper().dump();

    controller.handle_message(Box::new(factory.generate_stop_dumper_command()));

    assert_eq!(0.0, state.get_dumper().get_motor().get_speed());
}