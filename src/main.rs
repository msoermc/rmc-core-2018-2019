use crate::framework::logging::Logger;
use std::sync::mpsc::channel;
use crate::framework::Subsystem;
use std::thread;
use std::io::stdin;
use crate::framework::logging::LogType;
use crate::framework::logging::LogData;

pub mod framework;
pub mod subsystems;
pub mod devices;

fn main() {
    let (sender, receiver) = channel();
    let mut logger = Logger::new(sender);

    let log_channel = logger.get_command_sender();

    thread::spawn(move || logger.start());

    loop {
        let mut buffer = String::new();
        stdin().read_line(& mut buffer).unwrap();
        let timestamp = chrono::Utc::now();
        let severity = LogType::Info;

        let log = LogData::new(severity, timestamp, buffer);

        log_channel.send(log).unwrap();
    }
}
