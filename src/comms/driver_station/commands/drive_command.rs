use crate::comms::reading::Command;
use crate::comms::driver_station::DriverStationInterface;
use crate::drive_train::DriveTrainCommand;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;
use crate::comms::reading::rebuild_message;

pub struct DriveCommand {
    left_speed: f32,
    right_speed: f32,
}

impl Command<DriverStationInterface> for DriveCommand {
    fn accept(&self, interface: &DriverStationInterface) {
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

impl CommandReader<DriverStationInterface> for DriveCommandParser {
    fn read(&self, args: &[&str]) -> Result<Box<Command<DriverStationInterface>>, LogData> {
        let left_result = args[1].parse();
        let right_result = args[1].parse();

        match (left_result, right_result) {
            (Ok(left_speed), Ok(right_speed)) =>
                Ok(Box::new(DriveCommand::new(left_speed, right_speed))),
            (Err(left_error), Ok(right_speed)) => {
                let description = format!("Error: received invalid left argument in command '{}'", rebuild_message(args));
                Err(LogData::error(&description))
            }
            (Ok(left_speed), Err(right_error)) => {
                let description = format!("Error: received invalid right argument in command '{}'", rebuild_message(args));
                Err(LogData::error(&description))
            }
            (Err(left_error), Err(right_error)) => {
                let description = format!("Error: received invalid arguments in command '{}'", rebuild_message(args));
                Err(LogData::error(&description))
            }
        }
    }
}

impl DriveCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}