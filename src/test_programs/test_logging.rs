use std::{
    io::stdin,
    sync::mpsc::channel,
    thread,
};

use crate::{
    framework::{
        logging::{
            LogType,
            Logger,
            LogData,
        },
        Subsystem,
    }
};
use crate::subsystems::comms::Message;

pub fn run_test() {
    let (comms_sender, comms_receiver) = channel();
    let mut logger = Logger::new(comms_sender);

    let log_channel = logger.get_command_sender();

    thread::spawn(move || logger.start());

    thread::spawn(move || {
        loop {
            let Message::Log(log) = comms_receiver.recv().unwrap();
            println!("[COMMS]:\n{}", log.to_string());
        }
    });

    loop {
        let mut buffer = String::new();
        println!("Please enter a single-line log message.");
        stdin().read_line(&mut buffer).unwrap();
        let timestamp = chrono::Utc::now();
        let severity = LogType::Info;

        let log = LogData::new(severity, timestamp, buffer);

        log_channel.send(log).unwrap();
    }
}