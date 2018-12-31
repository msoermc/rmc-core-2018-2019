use crate::drive_train::DriveTrainCommand;
use crate::comms::driver_station::DriverStationInterface;
use crate::comms::reading::Command;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;

pub struct KillCommand {}
pub struct KillCommandParser {}

impl Command<DriverStationInterface> for KillCommand {
    fn accept(&self, interface: &DriverStationInterface) {
        interface.send_drive_train_command(DriveTrainCommand::Kill);
    }
}

impl KillCommand {
    pub fn new() -> Self {
        Self {}
    }
}


impl KillCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandReader<DriverStationInterface> for KillCommandParser {
    fn read(&self, args: &[&str]) -> Result<Box<Command<DriverStationInterface>>, LogData> {
        unimplemented!()
    }
}