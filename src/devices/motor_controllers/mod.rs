use crate::comms::SendableMessage;
use crate::robot_map::MotorID;

pub mod pwm;
pub mod test_motor;
pub mod motor_group;
pub mod print_motor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MotorStateKind {
    Unknown,
    Ok
}

impl ToString for MotorStateKind {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

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
    fn get_motor_state(&self) -> MotorState;
}

#[derive(Clone, Debug)]
pub struct MotorState {
    motor: MotorID,
    kind: MotorStateKind,
}

impl MotorState {
    pub fn new(motor: MotorID, kind: MotorStateKind) -> Self {
        MotorState { motor, kind }
    }

    pub fn get_motor(&self) -> MotorID {
        self.motor
    }

    pub fn get_state(&self) -> MotorStateKind {
        self.kind.clone()
    }
}

impl SendableMessage for MotorState {
    fn encode(&self) -> String {
        // TODO Add to protocol
        unimplemented!()
    }
}