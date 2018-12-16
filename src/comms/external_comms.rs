use std::{
    net::SocketAddr,
    sync::{
        mpsc::{
            Sender,
            Receiver,
        }
    },
    thread::spawn,
};

use tokio::{
    net::TcpListener,
    net::TcpStream,
    prelude::stream::Stream,
};

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
    clients: Vec<TcpStream>
}

impl ExternalComms {
    /// Instantiates the comms.
    /// This constructor will bind the listener.
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
            }
        };

        ExternalComms {
            sending_channel,
            logging_channel,
            listener,
            clients: Vec::new()
        }
    }

    /// Starts the comms in a <b>new</b> thread.
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
        unimplemented!()
    }

    fn send_messages(&mut self) {
        unimplemented!()
    }

    fn receive_messages(&mut self) {
        unimplemented!()
    }
}