use std::io;

/// Represents a single subsystem onboard the robot,
/// such as a trencher, bucket ladder, or chassis.
pub trait Subsystem {
    /// Initializes the subsystem.
    /// This function will run once before any subsystems are enabled.
    fn init<Err>(&mut self) -> Result<(), Err>;

    /// Enables the subsystem.
    /// While enabled, a subsystem will continuously run in it's own thread.
    fn enable<Err>(&mut self) -> Result<(), Err>;

    /// Disables the subsystem.
    /// While disabled, a subsystem does not run.
    fn disable<Err>(&mut self) -> Result<(), Err>;

    /// Kills the subsystem.
    /// Subsystem kills are meant to be invoked on all subsystems simultaneously.
    fn kill<Err>(&mut self) -> Result<(), Err>;

    /// Revives the subsystem.
    /// Subsystem revives are meant to be invoked on all subsystems simultaneously.
    fn revive<Err>(&mut self) -> Result<(), Err>;

    /// Returns true if the Subsystem is currently enabled and false otherwise.
    fn is_enabled(&self) -> bool;

    /// Returns the next message sent by the subsystem to the controller.
    fn receive_message<Message>(&mut self) -> Option<Message>;

    /// Sends a message to the subsystem.
    fn send_message<Message>(&mut self, message: Message) -> io::Result<()>;
}