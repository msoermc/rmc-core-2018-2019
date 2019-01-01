use crate::comms::driver_station::DriverStationController;
use crate::comms::get_wrong_arg_count_log;
use crate::comms::parsing::Command;
use crate::comms::parsing::CommandParser;
use crate::drive_train::DriveTrainCommand;
use crate::logging::log_data::LogData;

pub struct ReviveCommand {}

pub struct ReviveCommandParser {}

impl ToString for ReviveCommand {
    fn to_string(&self) -> String {
        format!("revive")
    }
}

impl<I> Command<I> for ReviveCommand where I: DriverStationController {
    fn execute(&self, interface: &I) {
        interface.send_drive_train_command(DriveTrainCommand::Revive);
    }
}

impl ReviveCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I> CommandParser<I> for ReviveCommandParser where I: DriverStationController {
    fn parse(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        if args.len() > 1 {
            Err(get_wrong_arg_count_log(args, 1, args.len() as u64))
        } else {
            Ok(Box::new(ReviveCommand::new()))
        }
    }
}

impl ReviveCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}