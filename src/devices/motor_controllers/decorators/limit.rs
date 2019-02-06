use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::devices::motor_controllers::GlobalMotorState;
use crate::devices::motor_controllers::MotorController;

pub struct FlagMotor<M: MotorController> {
    motor: M,
    disabled: AtomicBool,
}

impl<M: MotorController> MotorController for FlagMotor<M> {
    fn set_speed(&mut self, new_speed: f32) {
        if self.disabled.load(Ordering::SeqCst) {
            self.motor.stop();
        } else {
            self.motor.set_speed(new_speed);
        }
    }

    fn stop(&mut self) {
        self.motor.stop();
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        self.motor.get_motor_state()
    }
}

impl<M: MotorController> FlagMotor<M> {
    pub fn new(motor: M, disabled: AtomicBool) -> Self {
        Self {
            motor,
            disabled,
        }
    }
}