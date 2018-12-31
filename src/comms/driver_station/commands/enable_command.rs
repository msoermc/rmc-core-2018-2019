use crate::comms::driver_station::DriverStationInterface;
use crate::comms::driver_station::SubsystemIdentifier;
use crate::comms::reading::Command;
use crate::drive_train::DriveTrainCommand;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;

pub struct EnableCommand {
    subsystem: SubsystemIdentifier,
}

pub struct EnableCommandParser {}

impl<I> Command<I> for EnableCommand where I: DriverStationInterface {
    fn accept(&self, interface: &I) {
        match self.subsystem {
            SubsystemIdentifier::DriveTrainIdentifier =>
                interface.send_drive_train_command(DriveTrainCommand::Enable),
        }
    }
}

impl EnableCommand {
    pub fn new(subsystem: SubsystemIdentifier) -> Self {
        EnableCommand {
            subsystem,
        }
    }
}

impl EnableCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I> CommandReader<I> for EnableCommandParser where I: DriverStationInterface {
    fn read(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        unimplemented!()
    }
}