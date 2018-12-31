use crate::comms::reading::Command;
use crate::comms::driver_station::DriverStationInterface;
use crate::drive_train::DriveTrainCommand;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;
use crate::comms::get_wrong_arg_count_log;

pub struct ReviveCommand {}

pub struct ReviveCommandParser {}

impl<I> Command<I> for ReviveCommand where I: DriverStationInterface {
    fn accept(&self, interface: &I) {
        interface.send_drive_train_command(DriveTrainCommand::Revive);
    }
}

impl ReviveCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I> CommandReader<I> for ReviveCommandParser where I: DriverStationInterface {
    fn read(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
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