use crate::logging::log_data::LogData;
use std::collections::HashMap;

pub struct Command<RobotInterface> {
    handler: fn(&RobotInterface),
}

impl<RobotInterface> Command<RobotInterface> {
    pub fn accept(&self, interface: &RobotInterface) {
        let handler_fn = self.handler;

        handler_fn(interface)
    }

    pub fn new(handler: fn(&RobotInterface)) -> Self {
        Command {
            handler
        }
    }
}

pub struct CommandReader<RobotInterface> {
    reader: fn(&[&str]) -> Result<Command<RobotInterface>, LogData>,
}

impl<RobotInterface> CommandReader<RobotInterface> {
    pub fn read(&self, split_command: &[&str]) -> Result<Command<RobotInterface>, LogData> {
        let reader_fn = self.reader;

        reader_fn(split_command)
    }

    pub fn new(reader: fn(&[&str]) -> Result<Command<RobotInterface>, LogData>) -> Self {
        CommandReader {
            reader
        }
    }
}

pub struct Parser<RobotInterface> {
    readers: HashMap<String, CommandReader<RobotInterface>>,
}

impl<RobotInterface> Parser<RobotInterface> {
    pub fn parse(&self, message: &str) -> Result<Command<RobotInterface>, LogData> {
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

    pub fn new() -> Self {
        Parser {
            readers: HashMap::new()
        }
    }

    pub fn add_reader(&mut self, id: &str, reader: CommandReader<RobotInterface>) {
        if self.readers.insert(id.to_string(), reader).is_some() {
            panic!("Attempted to add duplicate reader!");
        }
    }
}