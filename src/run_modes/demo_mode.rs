use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::print_motor::PrintMotor;
use crate::run_modes::run_with_motors;

pub fn run_demo_mode() {
    let left_motor = Box::new(PrintMotor::new("Left"));
    let right_motor = Box::new(PrintMotor::new("Right"));

    let left_group = MotorGroup::new(vec![left_motor]);
    let right_group = MotorGroup::new(vec![right_motor]);

    run_with_motors(left_group, right_group);
}