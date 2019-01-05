use crate::comms::driver_station::DriverStationController;
use crate::comms::get_wrong_arg_count_log;
use crate::comms::parsing::Command;
use crate::comms::parsing::CommandParser;
use crate::drive_train::DriveTrainCommand;
use crate::logging::log_data::LogData;

pub struct KillCommand {}

pub struct KillCommandParser {}

impl ToString for KillCommand {
    fn to_string(&self) -> String {
        format!("kill")
    }
}

impl<I> Command<I> for KillCommand where I: DriverStationController {
    fn execute(&self, interface: &I) {
        interface.kill();
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

impl<I> CommandParser<I> for KillCommandParser where I: DriverStationController {
    fn parse(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        if args.len() > 1 {
            Err(get_wrong_arg_count_log(args, 1, args.len() as u64))
        } else {
            Ok(Box::new(KillCommand::new()))
        }
    }
}