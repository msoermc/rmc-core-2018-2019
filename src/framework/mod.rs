pub mod logging;

/// The subsystem trait represents different subsystems on the robot such as bucket ladders,
/// drivetrains, and dumping mechanisms.
///
/// Subsystems are run in their own processes concurrently by the robot framework.
pub trait Subsystem {
    /// Initializes the subsystem, returning a result object indicating whether the action was
    /// successful.
    fn init(&mut self);


    /// Runs a single loop of the subsystem. This function will be called repeatedly by the
    /// framework.
    fn run(&mut self);


    /// Enables the subsystem. The framework will run subsystems while they are enabled.
    ///
    /// Subsystems may only be enabled by the framework. They should not enable themselves.
    fn enable(&mut self);


    /// Disables the subsystem. The framework will not run subsystems while they are disabled.
    ///
    /// Subsystems should not disable themselves.
    fn disable(&mut self);


    /// Returns true if the subsystem is enabled and false otherwise.
    fn is_enabled(&self) -> bool;


    /// Represents an action to be run in a loop while a Subsystem is disabled.
    fn if_disabled(&mut self);
}

#[derive(Copy, Clone)]
pub enum TestMode {
    OnRobot(),
    Virtual(),
}