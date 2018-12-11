use std::sync::mpsc::Sender;

use crate::framework::{
    LogData,
    RobotError,
    Subsystem,
    TestMode,
};
use crate::devices::motor_controllers::{
    MotorController,
    hover_board::HoverBoardMotor,
};


pub struct DriveTrainError {}


impl RobotError for DriveTrainError {}


pub struct DriveTrain {
    test_mode: bool,
    is_enabled: bool,
    log_channel: Sender<LogData>,
    error_channel: Sender<DriveTrainError>,
    front_left: Box<MotorController>,
    front_right: Box<MotorController>,
    back_left: Box<MotorController>,
    back_right: Box<MotorController>,
}


impl Subsystem<DriveTrainError> for DriveTrain {
    fn init(&mut self) -> DriveTrainError {
        unimplemented!()
    }
    
    
    fn run(&mut self) {
        unimplemented!()
    }
    
    
    fn enable(&mut self) {
        self.is_enabled = true;
    }
    
    
    fn disable(&mut self) {
        self.is_enabled = false;
    }
    
    
    fn is_enabled(&self) -> bool {
        self.is_enabled
    }
    
    
    fn if_disabled(&mut self) {
        unimplemented!()
    }
}


impl DriveTrain {
    pub fn drive(left_speed: f32, right_speed: f32) {
        unimplemented!()
    }
    
    
    pub fn new(&mut self,
               logging_channel: Sender<LogData>,
               error_channel: Sender<DriveTrainError>,
               test_mode: TestMode)
        -> DriveTrain {
        unimplemented!()
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