use std::sync::mpsc::Sender;

use crate::comms::parsing::rebuild_message;
use crate::logging::log_data::LogData;

pub mod robot_communicator;

pub mod driver_station;
pub mod io;
mod parsing;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

fn get_wrong_arg_count_log(message: &[&str], expected: u64, actual: u64) -> LogData {
    let message = rebuild_message(message);
    let description = format!("Received wrong arg count to message '{}'. Expected {}, got {}!",
                              message, expected, actual);

    LogData::error(&description)
}

#[derive(Clone, Debug)]
pub struct CommsView {
    channel: Sender<Box<SendableMessage>>,
}

impl CommsView {
    pub fn send_message(&self, message: Box<SendableMessage>) -> Result<(), LogData> {
        match self.channel.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Comms sending channel hung up!")),
        }
    }

    pub fn new(channel: Sender<Box<SendableMessage>>) -> Self {
        Self {
            channel
        }
    }
}