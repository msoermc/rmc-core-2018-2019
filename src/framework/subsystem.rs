use std::sync::mpsc;

/// Represents a single subsystem on the robot,
/// such as a trencher, bucket ladder, or chassis.
///
/// The Command type parameter represents the type for messages sent
/// to the subsystem.
pub trait Subsystem {
    /// Initializes the subsystem.
    /// This function will run once before any subsystems are enabled.
    ///
    /// If successful, this function will return a tuple containing an mpsc
    /// sender which can be used to send messages to the subsystem and an
    /// mpsc receiver which can be used to receive messages sent by the
    /// subsystem.
    ///
    /// If unsuccessful, this method will return an InitError object.
    fn init<Command, ReportOK, ReportError, InitError>(&mut self) -> Result<
        (mpsc::Sender<Command>,
         mpsc::Receiver<Result<ReportOK, ReportError>>),
        InitError>;

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

/// Generates two pairs of mpsc sender and receiver objects.
/// The two pairs do not correspond to the channels they are in.
/// Instead, they are organized so that a thread can, through the usage of
/// one pair maintain two-way communication with a thread in possession of
/// another pair.
pub fn generate_channel_pair<Command, Report>() -> ((mpsc::Sender<Command>,
                                                     mpsc::Receiver<Report>),
                                                    (mpsc::Sender<Report>,
                                                     mpsc::Receiver<Command>)) {
    let command_channel = mpsc::channel();

    let report_channel = mpsc::channel();

    let command_report_pair = (command_channel.0, report_channel.1);

    let report_command_pair = (report_channel.0, command_channel.1);

    return (command_report_pair, report_command_pair);
}