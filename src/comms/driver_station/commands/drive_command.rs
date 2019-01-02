use crate::comms::driver_station::DriverStationController;
use crate::comms::get_wrong_arg_count_log;
use crate::comms::parsing::Command;
use crate::comms::parsing::CommandParser;
use crate::comms::parsing::rebuild_message;
use crate::drive_train::DriveTrainCommand;
use crate::logging::log_data::LogData;

pub struct DriveCommand {
    left_speed: f32,
    right_speed: f32,
}

impl ToString for DriveCommand {
    fn to_string(&self) -> String {
        format!("drive {} {}", self.left_speed, self.right_speed)
    }
}

impl<I> Command<I> for DriveCommand where I: DriverStationController {
    fn execute(&self, interface: &I) {
        interface.get_drive_interface().drive(self.left_speed, self.right_speed).unwrap();
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
        if args.len() == 3 {
            let left_result = args[1].parse();
            let right_result = args[2].parse();

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

#[cfg(test)]
mod tests {
    use crate::comms::driver_station::ConcreteDriverStationController;

    use super::*;

    #[test]
    fn test_valid() {
        let input_1 = ["drive", "1", ".5"];
        let input_2 = ["drive", "-1", "-.5"];

        let parser = DriveCommandParser::new();

        let actual_1: Box<Command<ConcreteDriverStationController>> = parser.parse(&input_1).unwrap();
        let actual_2: Box<Command<ConcreteDriverStationController>> = parser.parse(&input_2).unwrap();

        let expected_1 = DriveCommand::new(1.0, 0.5);
        let expected_2 = DriveCommand::new(-1.0, -0.5);

        assert_eq!(actual_1.to_string(), expected_1.to_string());
        assert_eq!(actual_2.to_string(), expected_2.to_string());
    }
}