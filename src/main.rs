use std::{
    sync::mpsc::channel,
    thread,
};

use crate::{
    framework::{
        logging::Logger,
        Subsystem,
    },
    subsystems::drive_train::DriveTrain,
};

pub mod framework;
pub mod subsystems;
pub mod devices;

fn main() {
    // Setup logger and get it's channels
    let logger = Logger::new();
    let log_sender = logger.get_sender();
    logger.start();

    // Setup DriveTrain and get it's channels
    let (drive_event_sender, drive_event_receiver) = channel();
    let mut drive_train = DriveTrain::new(log_sender, drive_event_sender);
    let drive_command_sender = drive_train.get_command_sender();

    let drive_thread = thread::spawn(move || {
        drive_train.init();
        loop {
            drive_train.run();
        }
    });

    drive_thread.join().unwrap();
}
