use crate::comms::driver_station::DriverStationInterface;
use crate::comms::reading::Command;
use crate::drive_train::DriveTrainCommand;

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