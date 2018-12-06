use std::sync::mpsc::Sender;
use crate::framework::{LogData, Subsystem, RobotError};

pub struct DriveTrainError {}

impl RobotError for DriveTrainError {}

pub struct DriveTrain {
    test_mode: bool,
}

impl Subsystem<DriveTrainError> for DriveTrain {
    fn init(&mut self, logging_channel: Sender<LogData>, error_channel: Sender<Box<DriveTrainError>>) -> DriveTrainError {
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

impl DriveTrain {
    pub fn drive(left_speed: f32, right_speed: f32) {
        unimplemented!()
    }
    pub fn new(test_mode: bool) -> DriveTrain {
        DriveTrain {test_mode}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        unimplemented!()
    }

    #[test]
    fn test_run() {
        unimplemented!()
    }

    #[test]
    fn test_disabled() {
        unimplemented!()
    }

    #[test]
    fn test_is_enabled() {
        unimplemented!()
    }

    #[test]
    fn test_if_disabled() {
        unimplemented!()
    }

    #[test]
    fn test_drive() {
        unimplemented!()
    }

    #[test]
    fn test_new() {
        unimplemented!()
    }
}