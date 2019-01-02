use crate::drive_train::DriveTrainCommand;
use std::sync::mpsc::Sender;
use crate::framework::interfaces::EnablingInterface;
use crate::logging::log_data::LogData;
use crate::framework::interfaces::RobotInterface;

#[derive(Clone, Debug)]
pub struct DriveTrainInterface {
    channel: Sender<DriveTrainCommand>,
}

impl EnablingInterface for DriveTrainInterface {
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

impl RobotInterface for DriveTrainInterface {

}

impl DriveTrainInterface {
    pub fn new(channel: Sender<DriveTrainCommand>) -> Self {
        DriveTrainInterface {
            channel,
        }
    }
}