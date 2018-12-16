use crate::{
    framework::{
        logging::{
            LogData,
            get_timestamp,
            LogType
        }
    }
};
use std::{
    net::{
        TcpStream,
        TcpListener
    },
    thread::spawn,
    sync::mpsc::Sender,
    collections::HashSet,
    io::ErrorKind::WouldBlock,
    error::Error
};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;

const ADDRESS: &str = "127.0.0.1:343";

pub enum SendableMessage {
    Log(LogData),
}

pub enum ProtocolSubsystem {
    DriveTrain,
}

enum ParsingError {

}

enum ReceivableMessage {
    Kill,
    Revive,
    Enable(ProtocolSubsystem),
    Disable(ProtocolSubsystem),
    Drive(f32, f32),
    Brake,
}

pub struct Communicator {
    sending_channel: Receiver<SendableMessage>,
    logging_channel: Sender<LogData>,
    listener: TcpListener,
    clients: Vec<TcpStream>,
}

impl Communicator {
    pub fn new(logging_channel: Sender<LogData>, sending_channel: Sender<SendableMessage>) -> Communicator {
        unimplemented!()
    }

    pub fn start(mut self) {
        spawn(move || {
            loop {
                self.run();
            }
        });
    }

    fn run(&mut self) {
        self.check_for_new_connections();
        self.receive_messages();
        self.send_messages();
    }

    fn check_for_new_connections(&mut self) {
        match self.listener.accept() {
            Ok((stream, _)) => self.clients.push(stream),
            Err(e) => {
                if e.kind() != WouldBlock {
                    let severity = LogType::Error;
                    let description = e.description().to_string();
                    let timestamp = get_timestamp();
                    LogData::new(severity, timestamp, description);
                }
            },
        };
    }

    fn send_messages(&mut self) {
        match self.sending_channel.try_recv() {
            Ok(message) => self.handle_new_sendable_message(message),
            Err(e) => {
                if TryRecvError::Disconnected == e {
                    self.handle_sending_channel_hangup();
                }
            },
        }
    }

    fn handle_sending_channel_hangup(&mut self) {
        let severity = LogType::Fatal;
        let description = "Sending channel for external comms hung up".to_string();
        let timestamp = get_timestamp();
        let log = LogData::new(severity, timestamp, description);
        self.logging_channel.send(log).unwrap(); // fail fast if the logger dies
    }

    fn receive_messages(&mut self) {
        unimplemented!()
    }

    fn handle_new_sendable_message(&mut self, message: SendableMessage) {
        unimplemented!()
    }

    fn handle_new_receivable_message(&mut self, message: ReceivableMessage) {
        match message {
            ReceivableMessage::Kill => self.handle_kill(),
            ReceivableMessage::Revive => self.handle_revive(),
            ReceivableMessage::Enable(subsystem) => self.handle_enable(subsystem),
            ReceivableMessage::Disable(subsystem) => self.handle_disable(subsystem),
            ReceivableMessage::Drive(left, right) => self.handle_drive(left, right),
            ReceivableMessage::Brake => self.handle_brake(),
        }
    }

    fn handle_kill(&mut self) {
        unimplemented!()
    }

    fn handle_revive(&mut self) {
        unimplemented!()
    }

    fn handle_enable(&mut self, subsystem: ProtocolSubsystem) {
        unimplemented!()
    }

    fn handle_disable(&mut self, subsystem: ProtocolSubsystem) {
        unimplemented!()
    }

    fn handle_brake(&mut self) {
        unimplemented!()
    }

    fn handle_drive(&mut self, left_speed: f32, right_speed: f32) {
        unimplemented!()
    }

    fn handle_log(&mut self) {
        unimplemented!()
    }
}

fn parse_receivable_message(message: String) -> Result<ReceivableMessage, ParsingError> {
    unimplemented!()
}