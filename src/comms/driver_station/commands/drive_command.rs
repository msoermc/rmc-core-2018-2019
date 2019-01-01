use crate::comms::parsing::Command;
use crate::comms::driver_station::DriverStationController;
use crate::drive_train::DriveTrainCommand;
use crate::comms::parsing::CommandParser;
use crate::logging::log_data::LogData;
use crate::comms::parsing::rebuild_message;
use crate::comms::get_wrong_arg_count_log;

pub struct DriveCommand {
    left_speed: f32,
    right_speed: f32,
}

impl<I> Command<I> for DriveCommand where I: DriverStationController {
    fn execute(&self, interface: &I) {
        let command = DriveTrainCommand::Drive(self.left_speed, self.right_speed);
        interface.send_drive_train_command(command);
    }
}

impl DriveCommand {
    pub fn new(left_speed: f32, right_speed: f32) -> Self {
        DriveCommand {
            left_speed,
            right_speed,
        }
    }
}

pub struct DriveCommandParser {}

impl<I> CommandParser<I> for DriveCommandParser where I: DriverStationController {
    fn parse(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        if args.len() != 3 {
            let left_result = args[1].parse();
            let right_result = args[1].parse();

            match (left_result, right_result) {
                (Ok(left_speed), Ok(right_speed)) =>
                    Ok(Box::new(DriveCommand::new(left_speed, right_speed))),
                (Err(_), Ok(_)) => {
                    let description = format!("Error: received invalid left argument in command '{}'", rebuild_message(args));
                    Err(LogData::error(&description))
                }
                (Ok(_), Err(_)) => {
                    let description = format!("Error: received invalid right argument in command '{}'", rebuild_message(args));
                    Err(LogData::error(&description))
                }
                (Err(_), Err(_)) => {
                    let description = format!("Error: received invalid arguments in command '{}'", rebuild_message(args));
                    Err(LogData::error(&description))
                }
            }
        } else {
            Err(get_wrong_arg_count_log(args, 3, args.len() as u64))
        }
    }
}

impl DriveCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}