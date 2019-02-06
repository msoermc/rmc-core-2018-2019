use crate::robot_map::MotorID;
use atomic::Atomic;
use atomic::Ordering as AtOrd;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub mod hover_board;
pub mod test_motor;
pub mod motor_group;
pub mod print_motor;

pub trait MotorController: Send {
    /// Sets the current speed of the motor controller.
    /// The speed should be a floating point number between -1 and 1.
    /// A negative speed indicates that the direction is reversed.
    fn set_speed(&mut self, new_speed: f32);

    /// Sets the current speed of the motor controller to zero.
    fn stop(&mut self);

    /// Inverts the directionality of the motor controller.
    fn invert(&mut self);

    /// Returns the current motor state
    fn get_motor_state(&self) -> &GlobalMotorState;
}

pub struct GlobalMotorState {
    value: Atomic<f32>,
    inverted: AtomicBool,
}

impl GlobalMotorState {
    pub fn new() -> Self {
        GlobalMotorState {
            value: Atomic::new(0.0),
            inverted: AtomicBool::new(false)
        }
    }

    pub fn get_current_state(&self) -> MotorStateInstance {
        MotorStateInstance::new(self.get_value(), self.get_inverted())
    }

    pub fn get_inverted(&self) -> bool {
        self.inverted.load(Ordering::Relaxed)
    }

    pub fn set_inverted(&self, inverted: bool) {
        self.inverted.store(inverted, Ordering::Relaxed)
    }

    pub fn get_value(&self) -> f32 {
        self.value.load(AtOrd::Relaxed)
    }

    pub fn set_value(&self, value: f32) {
        self.value.store(value, AtOrd::Relaxed);
    }
}

#[derive(Serialize)]
pub struct MotorStateInstance {
    value: f32,
    inverted: bool,
}

impl MotorStateInstance {
    pub fn new(value: f32, inverted: bool) -> Self {
        Self {
            value,
            inverted
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
}