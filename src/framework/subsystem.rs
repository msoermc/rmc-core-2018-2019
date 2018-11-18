use std::sync::mpsc;
use super::errors::*;

/// The subsystem trait represents different subsystems on the robot such as bucket ladders,
/// drivetrains, and dumping mechanisms.
///
/// Subsystems are run in their own processes concurrently by the robot framework.
pub trait Subsystem<S: OKStatus, R: RecoverableError, N: NonRecoverableError, I: InitError> {
    /// Initializes the subsystem, returning a result object indicating whether the action was
    /// successful.
    fn init(&mut self) -> Result<(), I>;

    /// Runs a single loop of the subsystem. This function will be called repeatedly by the
    /// framework.
    fn run(&mut self);

    /// Enables the subsystem. The framework will run subsystems while they are enabled.
    fn enable(&mut self) -> Result<(), RobotError<R, N>>;

    /// Disables the subsystem. The framework will not run subsystems while they are disabled.
    fn disable(&mut self) -> Result<(), RobotError<R, N>>;

    /// Returns true if the subsystem is enabled and false otherwise.
    fn is_enabled(&self) -> bool;

    fn get_status(&self) -> Result<S, RobotError<R, N>>;
}
