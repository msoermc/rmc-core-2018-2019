use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::test_motor::TestMotor;

use crate::devices::motor_controllers::GlobalMotorState;
use crate::devices::motor_controllers::motor_group::MotorGroup;

struct TestMotorGroup {
    pub state: Arc<GlobalMotorState>,
    pub motor_group: MotorGroup,
}

fn create_group() -> TestMotorGroup {
    let state = Arc::new(GlobalMotorState::new());

    let test_motor = TestMotor::new(state.clone());

    let test_group = MotorGroup::new(vec![Box::new(test_motor)]);

    TestMotorGroup { state, motor_group: test_group }
}

#[test]
fn test_set_speed_no_fail_no_inversion() {
    let mut group = create_group();

    // Test setup
    assert_eq!(0.0, group.state.get_speed());

    // Go forwards
    group.motor_group.set_speed(1.0);
    assert_eq!(1.0, group.state.get_speed());

    // Go backwards
    group.motor_group.set_speed(-1.0);
    assert_eq!(-1.0, group.state.get_speed());
}

#[test]
fn test_stop_no_fail_no_inversion() {
    let mut group = create_group();

    // Test setup
    assert_eq!(0.0, group.state.get_speed());

    // Go forwards
    group.motor_group.set_speed(1.0);
    assert_eq!(1.0, group.state.get_speed());

    // Stop
    group.motor_group.stop();
    assert_eq!(0.0, group.state.get_speed());
}