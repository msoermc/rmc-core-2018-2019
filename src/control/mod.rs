use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::RwLock;

use crate::logging::log_data::LogData;

pub mod controller;
pub mod drive_train;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RobotLifeStatus {
    Alive,
    Dead,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RobotControllerCommand {
    Drive(DriveCommandMessage),
    Brake,
    Enable,
    Disable,
}

pub struct RobotView {
    channel: Sender<RobotControllerCommand>,
    robot_life_status: Arc<RwLock<RobotLifeStatus>>,
}

impl RobotView {
    pub fn new(channel: Sender<RobotControllerCommand>, robot_life_status: Arc<RwLock<RobotLifeStatus>>) -> Self {
        Self {
            channel,
            robot_life_status,
        }
    }

    pub fn revive(&self) -> Result<(), LogData> {
        self.change_life_status(RobotLifeStatus::Alive)
    }

    pub fn kill(&self) -> Result<(), LogData> {
        self.change_life_status(RobotLifeStatus::Dead)
    }

    pub fn drive(&self, left: f32, right: f32) -> Result<(), LogData> {
        let command = DriveCommandMessage::new(left, right);

        self.send_command(RobotControllerCommand::Drive(command))
    }

    pub fn brake(&self) -> Result<(), LogData> {
        self.send_command(RobotControllerCommand::Brake)
    }

    pub fn enable_drive_train(&self) -> Result<(), LogData> {
        self.send_command(RobotControllerCommand::Enable)
    }

    pub fn disable_drive_train(&self) -> Result<(), LogData> {
        self.send_command(RobotControllerCommand::Disable)
    }

    fn send_command(&self, command: RobotControllerCommand) -> Result<(), LogData> {
        match self.channel.send(command) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Failed to send message to RobotController!")),
        }
    }

    fn change_life_status(&self, status: RobotLifeStatus) -> Result<(), LogData> {
        match self.robot_life_status.write() {
            Ok(mut flag) => {
                *flag = status;
                Ok(())
            }
            Err(_) => Err(LogData::fatal("Failed to revive robot!")),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DriveCommandMessage {
    left_speed: f32,
    right_speed: f32,
}

impl DriveCommandMessage {
    fn new(left_speed: f32, right_speed: f32) -> Self {
        if !check_speed(left_speed) && check_speed(right_speed) {
            panic!("Error in creating a DriveCommandMessage: left speed and right speed must be in range [-1, 1]!");
        } else {
            DriveCommandMessage { left_speed, right_speed }
        }
    }
}

fn check_speed(speed: f32) -> bool {
    speed <= 1.0 && speed >= -1.0
}