use crate::comms::driver_station::DriverStationInterface;
use crate::comms::driver_station::SubsystemIdentifier;
use crate::comms::reading::Command;
use crate::drive_train::DriveTrainCommand;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;

pub struct DisableCommand {
    subsystem: SubsystemIdentifier,
}

pub struct DisableCommandParser {}

impl Command<DriverStationInterface> for DisableCommand {
    fn accept(&self, interface: &DriverStationInterface) {
        match self.subsystem {
            SubsystemIdentifier::DriveTrainIdentifier =>
                interface.send_drive_train_command(DriveTrainCommand::Disable),
        }
    }
}

impl DisableCommand {
    pub fn new(subsystem: SubsystemIdentifier) -> Self {
        DisableCommand {
            subsystem,
        }
    }
}

impl DisableCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandReader<DriverStationInterface> for DisableCommandParser {
    fn read(&self, args: &[&str]) -> Result<Box<Command<DriverStationInterface>>, LogData> {
        unimplemented!()
    }
}