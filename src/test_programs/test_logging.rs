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
    },
    comms::SendableMessage,
};
use crate::comms::external_comms::ExternalComms;

pub fn run_test() {
    let (comms_sender, comms_receiver) = channel();
    let mut logger = Logger::new(comms_sender.clone());

    let log_channel = logger.get_command_sender();

    let comms = ExternalComms::new(log_channel.clone(), comms_receiver);

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