use crate::comms::Command;

enum DriverStationCommand {
    DriveCommand { left_speed: f32, right_speed: f32 },
    BrakeCommand,
    KillCommand,
    ReviveCommand,
    EnableCommand(SubsystemIdentifier),
    DisableCommand(SubsystemIdentifier)
}

enum SubsystemIdentifier {
    DriveTrainIdentifier
}