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

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

pub trait CommsController: LogAccepter {
    fn get_next_requested_send(&self) -> Option<Box<SendableMessage>>;
}

#[derive(Clone, Debug)]
pub struct CommsView {
    channel: Sender<Box<SendableMessage>>,
}

pub struct RobotCommunicator<R, I> where I: IoServerManager, R: CommsController {
    parser: MessageParser<R>,
    robot_interface: R,
    io: I,
}

impl CommsView {
    pub fn send_message(&self, message: Box<SendableMessage>) -> Result<(), LogData> {
        match self.channel.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Comms sending channel hung up!")),
        }
    }

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
            robot_interface,
            io,
        }
    }

    fn check_connection_statuses(&mut self) {
        if let Err(connection_status) = self.io.check_connections() {
            self.robot_interface.accept_log(connection_status);
        }
    }

    fn receive_messages(&mut self) {
        let messages_results = self.io.receive_next_lines();

        for message_result in messages_results {
            match message_result {
                Ok(message) => {
                    match self.parser.parse(&message) {
                        Ok(command) => command.execute(&self.robot_interface),
                        Err(log) => self.robot_interface.accept_log(log),
                    }
                }
                Err(log) => self.robot_interface.accept_log(log),
            }
        }
    }

    fn send_messages(&mut self) {
        if let Some(next_message) = self.robot_interface.get_next_requested_send() {
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