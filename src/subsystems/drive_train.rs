use std::io::Result;
use std::sync::mpsc::Sender;
use std::io::Error;

use crate::devices::Device;
use crate::devices::motor_controllers::{
    hover_board::HoverBoardMotor,
    MotorController,
};
use crate::framework::{
    logging::LogData,
    RobotError,
    Subsystem,
    TestMode,
};


pub struct DriveTrainError {}


impl RobotError for DriveTrainError {}


pub struct DriveTrain {
    test_mode: bool,
    is_enabled: bool,
    log_channel: Sender<LogData>,
    error_channel: Sender<DriveTrainError>,
    left: TankSide,
    right: TankSide,
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
    pub fn drive(&mut self, left_speed: f32, right_speed: f32) {
        self.right.set_speed(right_speed);
        self.left.set_speed(left_speed);
    }


    pub fn new(&mut self, logging_channel: Sender<LogData>,
               error_channel: Sender<DriveTrainError>, test_mode: TestMode) -> DriveTrain {
        unimplemented!()
    }
}


struct TankSide {
    is_inverted: bool,
    front: Box<MotorController>,
    back: Box<MotorController>,
}


impl MotorController for TankSide {
    fn set_speed(&mut self, new_speed: f32) -> Result<()> {
        let potentially_inverted_speed = if self.is_inverted() {
            -new_speed
        } else {
            new_speed
        };

        self.front.set_speed(potentially_inverted_speed)?;
        self.back.set_speed(potentially_inverted_speed)
    }

    fn stop(&mut self) -> Result<()> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) {
        self.is_inverted = !self.is_inverted()
    }

    fn is_inverted(&self) -> bool {
        self.is_inverted
    }
}


impl Device for TankSide {}