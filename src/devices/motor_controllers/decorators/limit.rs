use crate::devices::motor_controllers::MotorController;

pub struct LimitMotor<T: MotorController> {
    motor: T
}

impl MotorController for