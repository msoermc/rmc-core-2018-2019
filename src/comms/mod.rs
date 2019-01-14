use std::sync::mpsc::Sender;

use crate::comms::io::IoServerManager;
use crate::comms::parsing::MessageParser;
use crate::comms::parsing::rebuild_message;
use crate::framework::Runnable;
use crate::logging::log_data::LogData;
use crate::logging::LogAccepter;

pub mod driver_station;
pub mod io;
mod parsing;

/// A `SendableMessage` is an object that can be encoded as a message and sent off to another device.
pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

/// A `CommsController` is an object with methods that are called to run the Comms by the `RobotCommunicator`.
pub trait CommsController: LogAccepter {
    fn get_next_requested_send(&self) -> Option<Box<SendableMessage>>;
}

/// The `CommsView` is a view into a `RobotCommunicator` that other threads/objects
/// can use to request that messages be sent.
#[derive(Clone, Debug)]
pub struct CommsView {
    channel: Sender<Box<SendableMessage>>,
}

/// A `RobotCommunicator` is a complete Comms system which can be run.
/// It has a `CommsController` that it owns and a driver that it owns for io purposes.
/// The `C` type parameter is the type of the `CommsController`.
/// The `I` type parameter is the type of the `IoServerManager`.
pub struct RobotCommunicator<C, I> where I: IoServerManager, C: CommsController {
    parser: MessageParser<C>,
    controller: C,
    io: I,
}

impl CommsView {
    /// Sends a message to the remote receiver and returns `Err(LogData)` if the channel hangs up.
    pub fn send_message(&self, message: Box<SendableMessage>) -> Result<(), LogData> {
        match self.channel.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Comms sending channel hung up!")),
        }
    }

    /// Constructs a new `CommsView`
    pub fn new(channel: Sender<Box<SendableMessage>>) -> Self {
        Self {
            channel
        }
    }
}


impl<R, I> RobotCommunicator<R, I> where I: IoServerManager, R: CommsController {
    pub fn new(parser: MessageParser<R>, robot_interface: R, io: I) -> Self {
        RobotCommunicator {
            parser,
            controller: robot_interface,
            io,
        }
    }

    fn check_connection_statuses(&mut self) {
        if let Err(connection_status) = self.io.check_connections() {
            self.controller.accept_log(connection_status);
        }
    }

    fn receive_messages(&mut self) {
        let messages_results = self.io.receive_next_lines();

        for message_result in messages_results {
            match message_result {
                Ok(message) => {
                    match self.parser.parse(&message) {
                        Ok(command) => command.execute(&self.controller),
                        Err(log) => self.controller.accept_log(log),
                    }
                }
                Err(log) => self.controller.accept_log(log),
            }
        }
    }

    fn send_messages(&mut self) {
        if let Some(next_message) = self.controller.get_next_requested_send() {
            let encoding = next_message.encode();
            self.io.send_line(encoding);
        }
    }
}

impl<R, I> Runnable for RobotCommunicator<R, I> where I: IoServerManager, R: CommsController {
    fn init(&mut self) {
        //do nothing
    }

    fn run(&mut self) {
        self.check_connection_statuses();
        self.receive_messages();
        self.send_messages();
    }
}

fn get_wrong_arg_count_log(message: &[&str], expected: u64, actual: u64) -> LogData {
    let message = rebuild_message(message);
    let description = format!("Received wrong arg count to message '{}'. Expected {}, got {}!",
                              message, expected, actual);

    LogData::error(&description)
}