use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::robot_map::*;

use super::*;
use crate::mechatronics::material_handling::dumper::state::DUMPER_STOPPED;

struct TestMotorGroup {
    pub inverted: Arc<RwLock<bool>>,
    pub speed: Arc<RwLock<f32>>,
    pub motor_group: MotorGroup,
}

fn create_group() -> TestMotorGroup {
    let inverted = Arc::new(RwLock::new(false));
    let speed = Arc::new(RwLock::new(0.0));
    let test_motor = TestMotor::new(inverted.clone(), speed.clone());
    let test_group = MotorGroup::new(vec![Box::new(test_motor)]);
    TestMotorGroup { inverted, speed, motor_group: test_group }
}

#[test]
fn test_setup() {
    let group = create_group();
    let life = Arc::new(GlobalLifeState::new());
    let state = Arc::new(GlobalDumperState::new());

    let motor = group.motor_group;

    let dumper = Dumper::new(life.clone(), motor, state.clone());

    assert_eq!(false, state.get_enabled());
    assert_eq!(DUMPER_STOPPED, state.get_action());
    assert_eq!(0.0, *group.speed.read().unwrap());
}

#[test]
fn test_dumping() {

}