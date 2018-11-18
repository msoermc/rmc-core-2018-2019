use std::sync::mpsc;
use super::errors::*;

pub trait Subsystem<S, R: RecoverableError, N: NonRecoverableError, I: InitError> {
    fn init(&mut self) -> Result<(), I>;

    fn run(&mut self);

    fn enable(&mut self) -> Result<(), RobotError<R, N>>;

    fn disable(&mut self) -> Result<(), RobotError<R, N>>;

    fn is_enabled(&self) -> bool;

    fn get_status(&self) -> Result<S, RobotError<R, N>>;
}

/// Generates two pairs of mpsc sender and receiver objects.
/// The two pairs do not correspond to the channels they are in.
/// Instead, they are organized so that a thread can, through the usage of
/// one pair maintain two-way communication with a thread in possession of
/// another pair.
pub fn generate_channel_pair<ABMessage, BAMessage>() -> ((mpsc::Sender<ABMessage>,
                                                          mpsc::Receiver<BAMessage>),
                                                         (mpsc::Sender<BAMessage>,
                                                          mpsc::Receiver<ABMessage>)) {
    let command_channel = mpsc::channel();

    let report_channel = mpsc::channel();

    let command_report_pair = (command_channel.0, report_channel.1);

    let report_command_pair = (report_channel.0, command_channel.1);

    return (command_report_pair, report_command_pair);
}