use std::sync::mpsc::channel;

use crate::comms::driver_station::ExternalComms;
use crate::logging::get_timestamp;
use crate::logging::LogData;
use crate::logging::LogType;

pub fn run_test() {
    let (log_sender, _) = channel();
    let (comms_sender, comms_receiver) = channel();

    let (drive_sender, _) = channel();
    let comms = ExternalComms::new(log_sender, comms_receiver, drive_sender);

    comms.start();

    let mut counter: u64 = 0;
    loop {
        let description = format!("test {}\n", counter);
        let timestamp = get_timestamp();
        let severity = LogType::Debug;
        let log = LogData::new(severity, timestamp, description);
        comms_sender.send(Box::new(log)).expect("Could not send to comms");
        counter += 1;
    }
}