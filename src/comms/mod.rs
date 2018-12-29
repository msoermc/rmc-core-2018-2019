use crate::logging::log_data::LogData;
use std::collections::HashMap;

pub mod driver_station;
pub mod internal_comms;
mod io;
mod communicator;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

pub trait MessageParser<T>: Send {
    fn parse(&self, message: &str) -> Result<T, LogData>;
}

pub trait CommandParser<T>: Send {
    fn get_command(&self) -> &str;
    fn parse(&self, args: &[&str]) -> Result<T, LogData>;
}

pub trait CommandHandler<A> {
    fn handle(&mut self, args: A);
}

pub fn get_wrong_arg_count_log(message: &str, expected: u64, actual: u64) -> LogData {
    let description = format!(
        "Wrong number of elements in message '{}'. Expected {} args, instead got {}!",
        message, expected, actual);

    LogData::warning(description.as_str())
}

pub struct CommandMessageParser<T> {
    command_parsers: HashMap<String, Box<CommandParser<T>>>,
}

impl<T> MessageParser<T> for CommandMessageParser<T> {
    fn parse(&self, message: &str) -> Result<T, LogData> {
        let message = message.trim_end();   // Trim newline from end
        let elements: Vec<&str> = message.split_whitespace().collect();
        let command = match elements.first() {
            Some(com) => *com,
            None => {
                return Err(LogData::warning("Empty message in DS Comms!"));
            }
        };

        match self.command_parsers.get(command) {
            None => {
                let description = format!("Received nonexistent command, message is '{}'", message);
                Err(LogData::warning(description.as_str()))
            }
            Some(parser) => parser.parse(&elements),
        }
    }
}

impl<T> CommandMessageParser<T> {
    fn new() -> Box<Self> {
        Box::new(CommandMessageParser {
            command_parsers: HashMap::new()
        })
    }

    fn add_command(&mut self, parser: Box<CommandParser<T>>) {
        let command = parser.get_command();
        if self.command_parsers.contains_key(command) {
            panic!("Attempted to add duplicate parser!");
        } else {
            self.command_parsers.insert(command.to_string(), parser);
        }
    }
}