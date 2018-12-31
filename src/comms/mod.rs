use crate::logging::log_data::LogData;
use crate::comms::reading::rebuild_message;

pub mod command_io_controller;

pub mod driver_station;
mod io;
mod reading;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

fn get_wrong_arg_count_log(message: &[&str], expected: u64, actual: u64) -> LogData {
    let message = rebuild_message(message);
    let description = format!("Received wrong arg count to message '{}'. Expected {}, got {}!",
                              message, expected, actual);

    LogData::error(&description)
}