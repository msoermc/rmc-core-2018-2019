use crate::comms::SendableMessage;
use crate::logging::log_data::LogData;
use crate::robot_map::MotorID;

pub mod hover_board;
pub mod test_motor;
pub mod motor_group;
pub mod print_motor;

pub enum MotorFailureKind {
    Unknown,
    Thermal,
    Disconnect,
}

pub trait MotorController: Send {
    /// Sets the current speed of the motor controller.
    /// The speed should be a floating point number between -1 and 1.
    /// A negative speed indicates that the direction is reversed.
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure>;


    /// Sets the current speed of the motor controller to zero.
    fn stop(&mut self) -> Result<(), MotorFailure>;


    /// Inverts the directionality of the motor controller.
    fn invert(&mut self) -> Result<(), MotorFailure>;


    /// Returns true if the motor controller is inverted and false otherwise.
    fn is_inverted(&self) -> Result<bool, MotorFailure>;
}

pub struct MotorFailure {
    motor: MotorID,
    kind: MotorFailureKind,
    log: LogData,
}

impl MotorFailure {
    fn new(motor: MotorID, kind: MotorFailureKind, log: LogData) -> Self {
        MotorFailure { motor, kind, log }
    }
}

impl SendableMessage for MotorFailure {
    fn encode(&self) -> String {
        // TODO Add to protocol
        unimplemented!()
    }
}