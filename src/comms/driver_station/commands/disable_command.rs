use crate::comms::driver_station::DriverStationInterface;
use crate::comms::driver_station::SubsystemIdentifier;
use crate::comms::reading::Command;
use crate::drive_train::DriveTrainCommand;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;
use crate::comms::get_wrong_arg_count_log;

pub struct DisableCommand {
    subsystem: SubsystemIdentifier,
}

pub struct DisableCommandParser {}

impl<I> Command<I> for DisableCommand where I: DriverStationInterface {
    fn accept(&self, interface: &I) {
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

impl<I> CommandReader<I> for DisableCommandParser where I: DriverStationInterface {
    fn read(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        if args.len() > 2 {
            Err(get_wrong_arg_count_log(args, 2, args.len() as u64))
        } else {
            let subsystem = args[1].parse()?;
            Ok(Box::new(DisableCommand::new(subsystem)))
        }
    }
}