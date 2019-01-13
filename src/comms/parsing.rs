use std::collections::HashMap;

use crate::comms::CommsController;
use crate::logging::log_data::LogData;

pub trait Command<I>: ToString where I: CommsController {
    fn execute(&self, interface: &I);
}

pub trait CommandParser<I>: Send where I: CommsController {
    fn parse(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData>;
}

#[derive(Default)]
pub struct MessageParser<I> where I: CommsController {
    readers: HashMap<String, Box<CommandParser<I>>>
}

impl<I> MessageParser<I> where I: CommsController {
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

    pub fn add_reader(&mut self, id: &str, reader: Box<CommandParser<I>>) {
        if self.readers.insert(id.to_string(), reader).is_some() {
            panic!("Attempted to add duplicate reader!");
        }
    }
}

impl<I> MessageParser<I> where I: CommsController {
    pub fn new() -> Self {
        Self {
            readers: HashMap::new()
        }
    }
}

pub fn rebuild_message(args: &[&str]) -> String {
    args.iter().fold("".to_string(), |s0, s1| s0 + " " + s1).trim_start().to_string()
}