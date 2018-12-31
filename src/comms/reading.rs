use crate::logging::log_data::LogData;
use std::collections::HashMap;
use crate::comms::command_io_controller::RobotInterface;

pub trait Command<I> where I: RobotInterface {
    fn accept(&self, interface: &I);
}

pub trait CommandReader<I> where I: RobotInterface {
    fn read(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData>;
}

#[derive(Default)]
pub struct Parser<I> where I: RobotInterface {
    readers: HashMap<String, Box<CommandReader<I>>>
}

impl<I> Parser<I> where I: RobotInterface {
    pub fn parse(&self, message: &str) -> Result<Box<Command<I>>, LogData> {
        let split_message: Vec<&str> = message.split_whitespace().map(str::trim).collect();

        let parsed_command_id = split_message.first();

        match parsed_command_id {
            Some(id) => {
                match self.readers.get(*id) {
                    Some(reader) => reader.read(&split_message),
                    None => Err(LogData::error("Received unknown command!"))
                }
            }
            None => Err(LogData::error("Received empty command!"))
        }
    }

    pub fn add_reader(&mut self, id: &str, reader: Box<CommandReader<I>>) {
        if self.readers.insert(id.to_string(), reader).is_some() {
            panic!("Attempted to add duplicate reader!");
        }
    }
}

pub fn rebuild_message(args: &[&str]) -> String {
    args.iter().fold("".to_string(), |s0, s1| s0 + " " + s1)
}