use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::GlobalMotorState;

pub struct InvertedMotor<T: MotorController> {
    motor: T,
}

impl<T: MotorController> MotorController for InvertedMotor<T> {
    fn set_speed(&mut self, new_speed: f32) {
        self.motor.set_speed(-new_speed);
    }

    fn stop(&mut self) {
        self.motor.stop()
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        self.motor.get_motor_state()
    }
}

impl<T: MotorController> InvertedMotor<T> {
    pub fn new(motor: T) -> Self {
        Self {
            motor,
        }
    }
}