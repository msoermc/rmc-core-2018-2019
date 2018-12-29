use crate::comms::SendableMessage;
use std::sync::mpsc::Sender;
use crate::logging::log_data::LogData;
use crate::comms::communicator::Communicator;

pub mod sender;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 2401;

pub fn create_ds_comms(log_sender: Sender<LogData>, comms_sender: Sender<Box<SendableMessage>>)
                       -> Communicator<SendableMessage, ReceivableMessage>
{
    unimplemented!()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Subsystem {
    DriveTrain,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ReceivableMessage {
    Kill,
    Revive,
    Enable(Subsystem),
    Disable(Subsystem),
    Drive(f32, f32),
    Brake,
}