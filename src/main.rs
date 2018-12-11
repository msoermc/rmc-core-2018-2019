use crate::framework::{
    logging::*,
};
use crate::subsystems::{
    drive_train::*,
};
use std::sync::mpsc::channel;

pub mod framework;
pub mod subsystems;
pub mod devices;

fn main() {
    let logger = Logger::new();

    let log_sender = logger.get_sender();

    logger.start();

    let drive_train = DriveTrain::new(log_sender.clone(), channel().0);


}
