use crate::comms::driver_station::DriverStationController;
use crate::comms::driver_station::SubsystemIdentifier;
use crate::comms::get_wrong_arg_count_log;
use crate::comms::parsing::Command;
use crate::comms::parsing::CommandParser;
use crate::drive_train::DriveTrainCommand;
use crate::logging::log_data::LogData;

pub struct EnableCommand {
    subsystem: SubsystemIdentifier,
}

pub struct EnableCommandParser {}

impl ToString for EnableCommand {
    fn to_string(&self) -> String {
        format!("enable {}", self.subsystem.to_string())
    }
}

impl<I> Command<I> for EnableCommand where I: DriverStationController {
    fn execute(&self, interface: &I) {
        match self.subsystem {
            SubsystemIdentifier::DriveTrainIdentifier =>
                interface.get_drive_interface().enable().unwrap(),
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

impl<I> CommandParser<I> for EnableCommandParser where I: DriverStationController {
    fn parse(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        if args.len() > 2 {
            Err(get_wrong_arg_count_log(args, 2, args.len() as u64))
        } else {
            let subsystem = args[1].parse()?;
            Ok(Box::new(EnableCommand::new(subsystem)))
        }
    }
}