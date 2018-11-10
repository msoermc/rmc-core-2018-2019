use std::sync::mpsc;

/// Represents a single subsystem onboard the robot,
/// such as a trencher, bucket ladder, or chassis.
///
/// The CommandMessage type parameter represents the type for messages sent
/// to the subsystem.
///
/// The ReportingMessage type parameter represents the type
/// for messages received from the subsystem.
pub trait Subsystem<CommandMessage, ReportingMessage> {
    /// Initializes the subsystem.
    /// This function will run once before any subsystems are enabled.
    ///
    /// If successful, this function will return a tuple containing an mpsc
    /// sender which can be used to send messages to the subsystem and an
    /// mpsc receiver which can be used to receive messages sent by the
    /// subsystem.
    ///
    /// If unsuccessful, this method will return an Err object.
    fn init<Err>(&mut self) -> Result<(mpsc::Sender<CommandMessage>, mpsc::Receiver<ReportingMessage>), Err>;

    /// Enables the subsystem.
    /// While enabled, a subsystem will continuously run in it's own thread.
    fn enable<Err>(&mut self) -> Result<(), Err>;

    /// Disables the subsystem.
    /// While disabled, a subsystem does not run.
    fn disable<Err>(&mut self) -> Result<(), Err>;

    /// Returns true if the Subsystem is currently enabled and false otherwise.
    fn is_enabled(&self) -> bool;

    /// Kills the subsystem.
    /// Subsystem kills are meant to be invoked on all subsystems simultaneously.
    fn kill<Err>(&mut self) -> Result<(), Err>;

    /// Revives the subsystem.
    /// Subsystem revives are meant to be invoked on all subsystems simultaneously.
    fn revive<Err>(&mut self) -> Result<(), Err>;
}