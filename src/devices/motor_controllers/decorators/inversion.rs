use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::GlobalMotorState;

pub struct InvertedMotor {
    motor: Box<MotorController>,
}

impl MotorController for InvertedMotor {
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

impl InvertedMotor {
    pub fn new(motor: Box<MotorController>) -> Self {
        Self {
            motor,
        }
    }
}