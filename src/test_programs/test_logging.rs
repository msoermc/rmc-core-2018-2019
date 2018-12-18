use std::sync::mpsc::channel;
use std::thread;
use std::io::stdin;

use crate::comms::external_comms::ExternalComms;
use crate::framework::logging::Logger;
use crate::framework::Subsystem;
use crate::framework::logging::LogType;
use crate::framework::logging::LogData;

pub fn run_test() {
    let (comms_sender, comms_receiver) = channel();
    let mut logger = Logger::new(comms_sender.clone());
    let (drive_sender, drive_receiver) = channel();

    let log_channel = logger.get_command_sender();

    let comms = ExternalComms::new(log_channel.clone(), comms_receiver, drive_sender);

    comms.start();

    thread::spawn(move || logger.start());

    loop {
        let mut buffer = String::new();
        println!("Please enter a single-line log message.");
        stdin().read_line(&mut buffer).expect("problem on read line");
        let timestamp = chrono::Utc::now();
        let severity = LogType::Info;

        let log = LogData::new(severity, timestamp, buffer);

        comms_sender.send(Box::new(log)).expect("Problem sending typed message");
    }
}