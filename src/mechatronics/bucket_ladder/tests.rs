use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::GlobalMotorState;
use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::mechatronics::bucket_ladder::state::GlobalIntakeState;
use crate::robot_map::*;

use super::*;

struct TestMotorGroup {
    pub state: Arc<GlobalMotorState>,
    pub motor_group: Box<TestMotor>,
}

fn create_groups() -> (TestMotorGroup, TestMotorGroup) {
    let state_0 = Arc::new(GlobalMotorState::new());
    let state_1 = Arc::new(GlobalMotorState::new());

    let test_motor_0 = Box::new(TestMotor::new(state_0.clone()));
    let test_motor_1 = Box::new(TestMotor::new(state_1.clone()));

    let test_unit_0 = TestMotorGroup { state: state_0, motor_group: test_motor_0 };
    let test_unit_1 = TestMotorGroup { state: state_1, motor_group: test_motor_1 };

    (test_unit_0, test_unit_1)
}

#[test]
fn test_setup() {
    let (actuators, ladder) = create_groups();
    let actuators_m = actuators.motor_group;
    let ladder_m = ladder.motor_group;

    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalIntakeState::new());
    let intake = Intake::new(ladder_m, actuators_m, state.clone(), life.clone());

    assert_eq!(false, state.get_enabled());
    assert_eq!(0.0, actuators.state.get_speed());
    assert_eq!(0.0, ladder.state.get_speed());
}

#[test]
fn test_enabling() {
    let (actuators, ladder) = create_groups();
    let actuators_m = actuators.motor_group;
    let ladder_m = ladder.motor_group;

    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalIntakeState::new());
    let mut intake = Intake::new(ladder_m, actuators_m, state.clone(), life.clone());

    intake.enable();

    assert_eq!(true, state.get_enabled());
}

#[test]
fn test_raise() {
    let (actuators, ladder) = create_groups();
    let actuators_m = actuators.motor_group;
    let ladder_m = ladder.motor_group;

    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalIntakeState::new());
    let mut intake = Intake::new(ladder_m, actuators_m, state.clone(), life.clone());

    state.set_enabled(true);

    intake.raise();

    assert_eq!(MH_ACTUATOR_RATE, actuators.state.get_speed());

    intake.run_cycle();

    assert_eq!(MH_ACTUATOR_RATE, actuators.state.get_speed());

    intake.disable();

    assert_eq!(0.0, actuators.state.get_speed());

    intake.enable();

    intake.raise();

    assert_eq!(MH_ACTUATOR_RATE, actuators.state.get_speed());

    life.kill();

    intake.run_cycle();

    assert_eq!(0.0, actuators.state.get_speed());
    intake.raise();
    assert_eq!(0.0, actuators.state.get_speed());
}

#[test]
fn test_lower() {
    let (actuators, ladder) = create_groups();
    let actuators_m = actuators.motor_group;
    let ladder_m = ladder.motor_group;

    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalIntakeState::new());
    let mut intake = Intake::new(ladder_m, actuators_m, state.clone(), life.clone());

    state.set_enabled(true);

    intake.lower();

    assert_eq!(-MH_ACTUATOR_RATE, actuators.state.get_speed());

    intake.run_cycle();

    assert_eq!(-MH_ACTUATOR_RATE, actuators.state.get_speed());

    intake.disable();

    assert_eq!(0.0, actuators.state.get_speed());

    intake.enable();

    intake.lower();

    assert_eq!(-MH_ACTUATOR_RATE, actuators.state.get_speed());

    life.kill();

    intake.run_cycle();

    assert_eq!(0.0, actuators.state.get_speed());
    intake.lower();
    assert_eq!(0.0, actuators.state.get_speed());
}