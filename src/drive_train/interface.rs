use std::sync::mpsc::Sender;

use crate::drive_train::DriveTrainCommand;
use crate::framework::interfaces::EnablingInterface;
use crate::framework::interfaces::RobotInterface;
use crate::framework::interfaces::TankDriveInterface;
use crate::logging::log_data::LogData;

#[derive(Clone, Debug)]
pub struct ConcreteTankDriveInterface {
    channel: Sender<DriveTrainCommand>,
}

impl TankDriveInterface for ConcreteTankDriveInterface {
    fn drive(&self, left_speed: f32, right_speed: f32) -> Result<(), LogData> {
        match self.channel.send(DriveTrainCommand::Drive(left_speed, right_speed)) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Drive Train drive command hung up!")),
        }
    }

    fn brake(&self) -> Result<(), LogData> {
        match self.channel.send(DriveTrainCommand::Stop) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Drive Train stop command hung up!")),
        }
    }
}

impl EnablingInterface for ConcreteTankDriveInterface {
    fn enable(&self) -> Result<(), LogData> {
        match self.channel.send(DriveTrainCommand::Enable) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Drive Train enable command hung up!")),
        }
    }

    fn disable(&self) -> Result<(), LogData> {
        match self.channel.send(DriveTrainCommand::Disable) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Drive Train disable command hung up!")),
        }
    }
}

impl RobotInterface for ConcreteTankDriveInterface {}

impl ConcreteTankDriveInterface {
    pub fn new(channel: Sender<DriveTrainCommand>) -> Self {
        ConcreteTankDriveInterface {
            channel,
        }
    }
}