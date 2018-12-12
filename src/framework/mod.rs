use std::sync::mpsc::Sender;

pub mod logging;

/// The subsystem trait represents different subsystems on the robot such as bucket ladders,
/// drivetrains, and dumping mechanisms.
///
/// Subsystems are run in their own processes concurrently by the robot framework.
pub trait Subsystem<Command> {
    /// Initializes the subsystem, returning a result object indicating whether the action was
    /// successful.
    fn init(&mut self);

    /// Runs a single loop of the subsystem. This function will be called repeatedly by the
    /// framework.
    fn run(&mut self);

    fn get_command_sender(&mut self) -> Sender<Command>;
}

#[derive(Copy, Clone)]
pub enum TestMode {
    OnRobot(),
    Virtual(),
}