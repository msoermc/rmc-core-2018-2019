use std::collections::HashMap;

use crate::comms::CommsController;
use crate::logging::log_data::LogData;

/// A `Command` is a command received from a remote end which must be executed using a view of the robot.
pub trait Command<I>: ToString where I: CommsController {
    fn execute(&self, interface: &I);
}

/// A `CommandParser` is an object which parses the split fields of a message into a `Command`.
pub trait CommandParser<I>: Send where I: CommsController {
    fn parse(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData>;
}

/// Rebuilds a message from the split fields into its original encoded form.
pub fn rebuild_message(args: &[&str]) -> String {
    args.iter().fold("".to_string(), |s0, s1| s0 + " " + s1).trim_start().to_string()
}

/// A `MessageParser` is an object which maps id fields of messages to their associated `CommandParser`.
/// It can parse strings into `Command` objects.
/// `CommandParsers` are added dynamically.
#[derive(Default)]
pub struct MessageParser<I> where I: CommsController {
    readers: HashMap<String, Box<CommandParser<I>>>
}

impl<I> MessageParser<I> where I: CommsController {
    /// Parses a message into a command using the stored `CommandParser` objects.
    /// Returns `Err(LogData)` if the command is invalid.
    pub fn parse(&self, message: &str) -> Result<Box<Command<I>>, LogData> {
        let split_message: Vec<&str> = message.split_whitespace().map(str::trim).collect();

        let parsed_command_id = split_message.first();

        match parsed_command_id {
            Some(id) => {
                match self.readers.get(*id) {
                    Some(reader) => reader.parse(&split_message),
                    None => Err(LogData::error("Received unknown command!"))
                }
            }
            None => Err(LogData::error("Received empty command!"))
        }
    }

    /// Dynamically adds a new reader to the `MessageParser`.
    pub fn add_reader(&mut self, id: &str, reader: Box<CommandParser<I>>) {
        if self.readers.insert(id.to_string(), reader).is_some() {
            panic!("Attempted to add duplicate reader!");
        }
    }

    pub fn new() -> Self {
        Self {
            readers: HashMap::new()
        }
    }
}