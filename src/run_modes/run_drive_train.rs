use sysfs_gpio::Pin;
use sysfs_pwm::Pwm;

use crate::devices::motor_controllers::pwm::PwmMotor;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::robot_map::*;
use crate::run_modes::run_with_motors;

pub fn run_drive_train() {
    let left_front_pwm = Pwm::new(FRONT_LEFT_PWM_CHIP, FRONT_LEFT_PWM_NUMBER).expect("Front left pwm");
    let left_rear_pwm = Pwm::new(REAR_LEFT_PWM_CHIP, REAR_LEFT_PWM_NUMBER).expect("Rear left pwm");
    let right_front_pwm = Pwm::new(FRONT_RIGHT_PWM_CHIP, FRONT_RIGHT_PWM_NUMBER).expect("Front right pwm");
    let right_rear_pwm = Pwm::new(REAR_RIGHT_PWM_CHIP, REAR_RIGHT_PWM_NUMBER).expect("Rear right pwm");

    let front_right_direction = Pin::new(FRONT_RIGHT_DIRECTION);
    let front_left_direction = Pin::new(FRONT_LEFT_DIRECTION);
    let rear_right_direction = Pin::new(REAR_RIGHT_DIRECTION);
    let rear_left_direction = Pin::new(REAR_LEFT_DIRECTION);

    let front_right_motor = Box::new(PwmMotor::create(right_front_pwm, front_right_direction, MotorID::DriveTrainFrontRight).expect("Front right"));
    let front_left_motor = Box::new(PwmMotor::create(left_front_pwm, front_left_direction, MotorID::DriveTrainFrontLeft).expect("Front left"));
    let rear_right_motor = Box::new(PwmMotor::create(right_rear_pwm, rear_right_direction, MotorID::DriveTrainRearRight).expect("Rear right"));
    let rear_left_motor = Box::new(PwmMotor::create(left_rear_pwm, rear_left_direction, MotorID::DriveTrainRearLeft).expect("Rear left"));

    let left = MotorGroup::new(vec![front_left_motor, rear_left_motor]);
    let right = MotorGroup::new(vec![front_right_motor, rear_right_motor]);

    run_with_motors(left, right);
}