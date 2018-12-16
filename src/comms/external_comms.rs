use std::{
    net::SocketAddr,
    sync::{
        mpsc::{
            Sender,
            Receiver,
            TryRecvError,
        }
    },
    thread::spawn,
};
use std::io::BufReader;

use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::{
    framework::{
        logging::{
            LogData,
            get_timestamp,
            LogType,
        }
    }
};

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 343;

pub enum SendableMessage {
    Log(LogData),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProtocolSubsystem {
    DriveTrain,
}

enum ParsingError {}

#[derive(Copy, Clone)]
enum ReceivableMessage {
    Kill,
    Revive,
    Enable(ProtocolSubsystem),
    Disable(ProtocolSubsystem),
    Drive(f32, f32),
    Brake,
}

pub struct ExternalComms {
    sending_channel: Receiver<SendableMessage>,
    logging_channel: Sender<LogData>,
    listener: TcpListener,
    clients: Vec<BufReader<TcpStream>>,
}

impl ExternalComms {
    pub fn new(logging_channel: Sender<LogData>, sending_channel: Receiver<SendableMessage>) -> ExternalComms {
        let address = SocketAddr::new(
            ADDRESS.parse().expect("Could not parse address"),
            PORT);

        let listener = match TcpListener::bind(&address) {
            Ok(lis) => lis,
            Err(_) => {
                let description = "Could not bind listener for external comms!";
                let timestamp = get_timestamp();
                let severity = LogType::Fatal;

                let log = LogData::new(severity, timestamp, description.to_string());

                let could_log = logging_channel.send(log).is_ok();

                if could_log {
                    panic!(description);
                } else {
                    panic!("Could not bind listener for external comms and could not log either!");
                }
            },
        };

        let clients = Vec::new();

        ExternalComms {
            sending_channel,
            logging_channel,
            listener,
            clients,
        }
    }

    pub fn start(mut self) {
        spawn(move || {
            loop {
                self.run();
            }
        });
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Running loop
    ///////////////////////////////////////////////////////////////////////////////////////////////
    fn run(&mut self) {
        self.check_for_new_connections();
        self.receive_messages();
        self.send_messages();
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Checking for new connections
    ///////////////////////////////////////////////////////////////////////////////////////////////
    fn check_for_new_connections(&mut self) {
        unimplemented!()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Sending messages
    ///////////////////////////////////////////////////////////////////////////////////////////////
    fn send_messages(&mut self) {
        match self.sending_channel.try_recv() {
            Ok(message) => self.handle_new_sendable_message(message),
            Err(e) => {
                if TryRecvError::Disconnected == e {
                    self.handle_sending_channel_hangup();
                }
            }
        }
    }

    fn handle_sending_channel_hangup(&mut self) {
        let severity = LogType::Fatal;
        let description = "Sending channel for external comms hung up".to_string();
        let timestamp = get_timestamp();
        let log = LogData::new(severity, timestamp, description);
        self.logging_channel.send(log).unwrap(); // fail fast if the logger dies
    }

    fn handle_new_sendable_message(&mut self, message: SendableMessage) {
        unimplemented!()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Receiving messages
    ///////////////////////////////////////////////////////////////////////////////////////////////
    fn receive_messages(&mut self) {
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