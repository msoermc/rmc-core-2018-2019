use crate::comms::driver_station::DriverStationController;
use crate::comms::get_wrong_arg_count_log;
use crate::comms::parsing::Command;
use crate::comms::parsing::CommandParser;
use crate::drive_train::DriveTrainCommand;
use crate::logging::log_data::LogData;

pub struct BrakeCommand {}

pub struct BrakeCommandParser {}

impl ToString for BrakeCommand {
    fn to_string(&self) -> String {
        format!("brake")
    }
}

impl<I> Command<I> for BrakeCommand where I: DriverStationController {
    fn execute(&self, interface: &I) {
        let command = DriveTrainCommand::Stop;
        interface.send_drive_train_command(command);
    }
}

impl BrakeCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BrakeCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I> CommandParser<I> for BrakeCommandParser where I: DriverStationController {
    fn parse(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        if args.len() > 1 {
            Err(get_wrong_arg_count_log(args, 1, args.len() as u64))
        } else {
            Ok(Box::new(BrakeCommand::new()))
        }
    }
}