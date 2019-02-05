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