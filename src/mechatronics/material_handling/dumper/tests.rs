use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::robot_map::*;

use super::*;

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