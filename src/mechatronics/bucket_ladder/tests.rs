use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::mechatronics::bucket_ladder::state::GlobalIntakeState;
use crate::robot_map::*;

use super::*;

struct TestMotorGroup {
    pub inverted: Arc<RwLock<bool>>,
    pub speed: Arc<RwLock<f32>>,
    pub motor_group: MotorGroup,
}

fn create_groups() -> (TestMotorGroup, TestMotorGroup) {
    let inverted_0 = Arc::new(RwLock::new(false));
    let inverted_1 = Arc::new(RwLock::new(false));

    let speed_0 = Arc::new(RwLock::new(0.0));
    let speed_1 = Arc::new(RwLock::new(0.0));

    let test_motor_0 = TestMotor::new(inverted_0.clone(), speed_0.clone());
    let test_motor_1 = TestMotor::new(inverted_1.clone(), speed_1.clone());

    let test_group_0 = MotorGroup::new(vec![Box::new(test_motor_0)]);
    let test_group_1 = MotorGroup::new(vec![Box::new(test_motor_1)]);

    let test_unit_0 = TestMotorGroup { inverted: inverted_0, speed: speed_0, motor_group: test_group_0 };
    let test_unit_1 = TestMotorGroup { inverted: inverted_1, speed: speed_1, motor_group: test_group_1 };

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
    assert_eq!(0.0, *actuators.speed.read().unwrap());
    assert_eq!(0.0, *ladder.speed.read().unwrap());
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

    assert_eq!(MH_ACTUATOR_RATE, *actuators.speed.read().unwrap());

    intake.run_cycle();

    assert_eq!(MH_ACTUATOR_RATE, *actuators.speed.read().unwrap());

    intake.disable();

    assert_eq!(0.0, *actuators.speed.read().unwrap());

    intake.enable();

    intake.raise();

    assert_eq!(MH_ACTUATOR_RATE, *actuators.speed.read().unwrap());

    life.kill();

    intake.run_cycle();

    assert_eq!(0.0, *actuators.speed.read().unwrap());
    intake.raise();
    assert_eq!(0.0, *actuators.speed.read().unwrap());
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

    assert_eq!(-MH_ACTUATOR_RATE, *actuators.speed.read().unwrap());

    intake.run_cycle();

    assert_eq!(-MH_ACTUATOR_RATE, *actuators.speed.read().unwrap());

    intake.disable();

    assert_eq!(0.0, *actuators.speed.read().unwrap());

    intake.enable();

    intake.lower();

    assert_eq!(-MH_ACTUATOR_RATE, *actuators.speed.read().unwrap());

    life.kill();

    intake.run_cycle();

    assert_eq!(0.0, *actuators.speed.read().unwrap());
    intake.lower();
    assert_eq!(0.0, *actuators.speed.read().unwrap());
}