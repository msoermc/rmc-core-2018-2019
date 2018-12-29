use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread::spawn;

use crate::comms::SendableMessage;
use crate::logging::log_data::LogData;
use crate::logging::log_sender::LogSender;
use crate::logging::LogAccepter;
use crate::comms::io::tcp::TcpIoManager;
use crate::comms::io::IoConnectionManager;
use crate::comms::MessageParser;
use crate::comms::CommandParser;

pub struct Communicator<S: SendableMessage, R> {
    message_queue: Receiver<Box<S>>,
    log_sender: LogSender,
    communicator: TcpIoManager,
    parser: Box<MessageParser<R>>,
}

impl<S: SendableMessage + 'static, R: 'static> Communicator<S, R> {
    pub fn new(address: &str, port: u16, logging_channel: LogSender, message_queue: Receiver<Box<S>>,
               parser: Box<MessageParser<R>>) -> Box<Communicator<S, R>> {
        let communicator = TcpIoManager::new(address, port);

        Box::new(Communicator {
            message_queue,
            log_sender: logging_channel,
            communicator,
            parser,
        })
    }

    pub fn start(mut self) {
        spawn(move || {
            loop {
                self.run();
            }
        });
    }

    fn run(&mut self) {
        self.check_connections();
        self.receive_messages();
        self.send_messages();
    }

    fn check_connections(&mut self) {
        if let Err(error) = self.communicator.check_connections() {
            self.log_sender.accept_log(error);
        }
    }

    fn send_messages(&mut self) {
        match self.message_queue.try_recv() {
            Ok(message) => self.send_message(message.as_ref()),
            Err(try_error) => {
                if let TryRecvError::Disconnected = try_error {
                    self.handle_sending_channel_disconnect();
                }
            }
        }
    }

    fn receive_messages(&mut self) {
        for result in self.communicator.receive_next_lines() {
            match result {
                Ok(message) => self.handle_message(message.as_str()),
                Err(error) => self.log_sender.accept_log(error)
            }
        }
    }

    fn handle_message(&mut self, message: &str) {
        let parsed_result = self.parser.parse(message);
        match parsed_result {
            Ok(parsed_message) => self.handle_valid_command(parsed_message),
            Err(log) => self.log_sender.accept_log(log)
        }
    }

    fn send_message(&mut self, message: &SendableMessage) {
        let sending_string = message.encode();

        let sending_logs = self.communicator.send_line(sending_string);

        for log in sending_logs {
            self.log_sender.accept_log(log)
        }
    }

    fn handle_sending_channel_disconnect(&mut self) {
        let log = LogData::fatal("Sending channel disconnected in external comms!");
        self.log_sender.accept_log(log);
        panic!("Sending channel disconnected in external comms!");
    }

    fn handle_valid_command(&mut self, message: R) {
        unimplemented!()
    }
}
