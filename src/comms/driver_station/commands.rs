use crate::comms::driver_station::DriverStationInterface;
use crate::comms::reading::Command;
use crate::drive_train::DriveTrainCommand;
use crate::comms::driver_station::SubsystemIdentifier;

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

pub struct BrakeCommand {}

impl Command<DriverStationInterface> for BrakeCommand {
    fn accept(&self, interface: &DriverStationInterface) {
        let command = DriveTrainCommand::Stop;
        interface.send_drive_train_command(command);
    }
}

impl BrakeCommand {
    pub fn new() -> Self {
        BrakeCommand {}
    }
}

pub struct KillCommand {}

impl Command<DriverStationInterface> for KillCommand {
    fn accept(&self, interface: &DriverStationInterface) {
        interface.send_drive_train_command(DriveTrainCommand::Kill);
    }
}

impl KillCommand {
    pub fn new() -> Self {
        KillCommand {}
    }
}

pub struct ReviveCommand {}

impl Command<DriverStationInterface> for ReviveCommand {
    fn accept(&self, interface: &DriverStationInterface) {
        interface.send_drive_train_command(DriveTrainCommand::Revive);
    }
}

impl ReviveCommand {
    pub fn new() -> Self {
        ReviveCommand {}
    }
}

pub struct EnableCommand {
    subsystem: SubsystemIdentifier,
}

impl Command<DriverStationInterface> for EnableCommand {
    fn accept(&self, interface: &DriverStationInterface) {
        match self.subsystem {
            SubsystemIdentifier::DriveTrainIdentifier =>
                interface.send_drive_train_command(DriveTrainCommand::Enable),
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

pub struct DisableCommand {
    subsystem: SubsystemIdentifier,
}

impl Command<DriverStationInterface> for DisableCommand {
    fn accept(&self, interface: &DriverStationInterface) {
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