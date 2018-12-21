use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;

use crate::comms::driver_station::ExternalComms;
use crate::framework::Runnable;
use crate::logging::LogData;
use crate::logging::Logger;
use crate::logging::LogType;

pub fn run_test() {
    let (comms_sender, comms_receiver) = channel();
    let (log_sender, log_receiver) = channel();

    let mut logger = Logger::new(comms_sender.clone(), log_receiver);
    let (drive_sender, _) = channel();


    let comms = ExternalComms::new(log_sender.clone(), comms_receiver, drive_sender);

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