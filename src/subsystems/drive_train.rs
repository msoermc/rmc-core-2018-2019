use std::sync::mpsc::Sender;
use crate::framework::{LogData, Subsystem, RobotError};

pub enum DriveTrainError {}

impl RobotError for DriveTrainError {

}

pub struct DriveTrain {}

impl Subsystem<DriveTrainError> for DriveTrain {
    fn init(&mut self, logging_channel: Sender<Box<LogData>>, error_channel: Sender<Box<DriveTrainError>>) -> DriveTrainError {
        unimplemented!()
    }

    fn run(&mut self) {
        unimplemented!()
    }

    fn enable(&mut self) {
        unimplemented!()
    }

    fn disable(&mut self) {
        unimplemented!()
    }

    fn is_enabled(&self) -> bool {
        unimplemented!()
    }

    fn if_disabled(&mut self) {
        unimplemented!()
    }
}