use crate::drive_train::DriveTrainCommand;
use crate::comms::driver_station::DriverStationInterface;
use crate::comms::reading::Command;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;

pub struct KillCommand {}

pub struct KillCommandParser {}

impl<I> Command<I> for KillCommand where I: DriverStationInterface {
    fn accept(&self, interface: &I) {
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

impl<I> CommandReader<I> for KillCommandParser where I: DriverStationInterface {
    fn read(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        unimplemented!()
    }
}