use crate::logging::log_data::LogData;
use std::sync::mpsc::Sender;
use std::sync::mpsc::SendError;
use crate::logging::LogAccepter;

#[derive(Debug, Clone)]
pub struct LogSender {
    logging_channel: Sender<LogData>
}

impl LogSender {
    pub fn new(logging_channel: Sender<LogData>) -> Self {
        LogSender {
            logging_channel
        }
    }

    pub fn send_debug(&self, description: &str) -> Result<(), SendError<LogData>> {
        self.logging_channel.send(LogData::debug(description))
    }

    pub fn send_info(&self, description: &str) -> Result<(), SendError<LogData>> {
        self.logging_channel.send(LogData::info(description))
    }

    pub fn send_warning(&self, description: &str) -> Result<(), SendError<LogData>> {
        self.logging_channel.send(LogData::warning(description))
    }

    pub fn send_error(&self, description: &str) -> Result<(), SendError<LogData>> {
        self.logging_channel.send(LogData::error(description))
    }

    pub fn send_fatal(&self, description: &str) -> Result<(), SendError<LogData>> {
        self.logging_channel.send(LogData::fatal(description))
    }
}

impl LogAccepter for LogSender {
    fn accept_log(&mut self, log: LogData) {
        self.logging_channel.send(log).unwrap()
    }
}