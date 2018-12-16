use std::{
    net::{
        SocketAddr,
        TcpListener,
        TcpStream,
    },
    sync::{
        mpsc::{
            Sender,
            Receiver,
            TryRecvError,
        }
    },
    thread::spawn,
    io::Write
};

use crate::{
    comms::SendableMessage,
    framework::{
        logging::{
            LogData,
            get_timestamp,
            LogType,
        }
    },
};
use std::io::ErrorKind;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 2401;

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
    sending_channel: Receiver<Box<SendableMessage>>,
    logging_channel: Sender<LogData>,
    listener: TcpListener,
    clients: Vec<TcpStream>,
}

impl ExternalComms {
    /// Instantiates the comms.
    /// This constructor will bind the listener.
    pub fn new(logging_channel: Sender<LogData>, sending_channel: Receiver<Box<SendableMessage>>) -> ExternalComms {
        let address = SocketAddr::new(
            ADDRESS.parse().expect("Could not parse address"),
            PORT);

        let listener = TcpListener::bind(&address).unwrap();

        listener.set_nonblocking(true).expect("Could not set listener to be nonblocking!");

        ExternalComms {
            sending_channel,
            logging_channel,
            listener,
            clients: Vec::new(),
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
        let connection_result = self.listener.accept();
        match connection_result {
            Ok(potential_connection) => {
                let (socket, _) = potential_connection;
                self.clients.push(socket);
            }
            Err(error) => {
                if error.kind() != ErrorKind::WouldBlock {
                    self.handle_lost_listener();
                }
            }
        }
    }

    fn send_messages(&mut self) {
        match self.sending_channel.try_recv() {
            Ok(message) => {}
            Err(try_error) => {
                if let TryRecvError::Disconnected = try_error {
                    self.handle_sending_channel_disconnect();
                }
            }
        }
    }

    fn receive_messages(&mut self) {
        unimplemented!()
    }

    fn handle_lost_listener(&mut self) {
        unimplemented!()
    }

    fn send_message(&mut self, message: Box<SendableMessage>) {
        let sending_string = message.encode();

        for client in &mut self.clients {
            writeln!(client, "{}", sending_string);
        }
    }

    fn handle_sending_channel_disconnect(&mut self) {
        let timestamp = get_timestamp();
        let severity = LogType::Fatal;
        let description = "Sending channel disconnected in external comms!";
        let log = LogData::new(severity, timestamp, description.to_string());
        self.logging_channel.send(log).expect("Sending and logging channels disconnected in external comms");
        panic!("{}", "Sending channel disconnected in external comms!");
    }
}