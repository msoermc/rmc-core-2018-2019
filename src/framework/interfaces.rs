use crate::logging::log_data::LogData;
use crate::comms::SendableMessage;

pub trait RobotInterface: Clone {

}

pub trait EnablingInterface: RobotInterface {
    fn enable(&self) -> Result<(), LogData>;
    fn disable(&self) -> Result<(), LogData>;
}

pub trait TankDriveInterface: EnablingInterface + RobotInterface {
    fn drive(&self, left_speed: f32, right_speed: f32) -> Result<(), LogData>;
    fn brake(&self) -> Result<(), LogData>;
}

pub trait CommunicationsInterface: RobotInterface {
    fn send_message(&self, message: Box<SendableMessage>) -> Result<(), LogData>;
}