use crate::comms::reading::Command;
use crate::comms::driver_station::DriverStationInterface;
use crate::drive_train::DriveTrainCommand;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;

pub struct ReviveCommand {}

pub struct ReviveCommandParser {}

impl Command<DriverStationInterface> for ReviveCommand {
    fn accept(&self, interface: &DriverStationInterface) {
        interface.send_drive_train_command(DriveTrainCommand::Revive);
    }
}

impl ReviveCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandReader<DriverStationInterface> for ReviveCommandParser {
    fn read(&self, args: &[&str]) -> Result<Box<Command<DriverStationInterface>>, LogData> {
        unimplemented!()
    }
}

impl ReviveCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}