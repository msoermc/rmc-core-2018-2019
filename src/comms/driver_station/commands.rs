use crate::comms::driver_station::SubsystemIdentifier;

struct DriveCommand {
    left_speed: f32,
    right_speed: f32
}

struct BrakeCommand {

}

struct KillCommand {

}

struct ReviveCommand {

}

struct DisableCommand {
    subsystem: SubsystemIdentifier
}

struct EnableCommand {
    subsystem: SubsystemIdentifier
}