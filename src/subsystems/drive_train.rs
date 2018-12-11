use std::io::Result;
use std::sync::mpsc::Sender;

use crate::devices::{
    Device,
    motor_controllers::MotorController,
};
use crate::framework::{
    logging::LogData,
    Subsystem,
};

pub enum DriveTrainState {
    RightSideMotorError(),
    LeftSideMotorError(),
}

pub struct DriveTrain {
    is_enabled: bool,
    log_channel: Sender<LogData>,
    error_channel: Sender<DriveTrainState>,
    left: TankSide,
    right: TankSide,
}


impl Subsystem for DriveTrain {
    fn init(&mut self) {
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
        let right_result = self.right.set_speed(right_speed);
        let left_result = self.left.set_speed(left_speed);

        match right_result {
            Ok(_) => (),
            Err(error) => unimplemented!(),
        };

        match left_result {
            Ok(_) => (),
            Err(error) => unimplemented!(),
        };
    }


    pub fn new(log_channel: Sender<LogData>, error_channel: Sender<DriveTrainState>) -> DriveTrain {
        DriveTrain {
            is_enabled: true,
            log_channel,
            error_channel,
            left: get_left_side(),
            right: get_right_side(),
        }
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

fn get_left_side() -> TankSide {
    unimplemented!()
}

fn get_right_side() -> TankSide {
    unimplemented!()
}


impl Device for TankSide {}