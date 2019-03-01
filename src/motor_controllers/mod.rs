use atomic::Atomic;
use atomic::Ordering as AtOrd;

pub mod hover_board;
pub mod test_motor;
pub mod motor_group;
pub mod print_motor;
pub mod decorators;

pub trait MotorController: Send {
    /// Sets the current speed of the motor controller.
    /// The speed should be a floating point number between -1 and 1.
    /// A negative speed indicates that the direction is reversed.
    fn set_speed(&mut self, new_speed: f32);

    /// Sets the current speed of the motor controller to zero.
    fn stop(&mut self);

    /// Returns the current motor state
    fn get_motor_state(&self) -> &GlobalMotorState;
}

pub struct GlobalMotorState {
    speed: Atomic<f32>,
}

impl GlobalMotorState {
    pub fn new() -> Self {
        GlobalMotorState {
            speed: Atomic::new(0.0),
        }
    }

    pub fn get_current_state(&self) -> MotorStateInstance {
        MotorStateInstance::new(self.get_speed())
    }

    pub fn get_speed(&self) -> f32 {
        self.speed.load(AtOrd::Relaxed)
    }

    pub fn set_speed(&self, value: f32) {
        self.speed.store(value, AtOrd::Relaxed);
    }
}

#[derive(Serialize)]
pub struct MotorStateInstance {
    speed: f32,
}

impl MotorStateInstance {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}