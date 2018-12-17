use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread::spawn;

use crate::comms::Communicator;
use crate::comms::CommunicatorError;
use crate::comms::SendableMessage;
use crate::framework::logging::get_timestamp;
use crate::framework::logging::LogData;
use crate::framework::logging::LogType;
use crate::subsystems::drive_train::DriveTrainCommand;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 2401;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProtocolSubsystem {
    DriveTrain,
}

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
    communicator: Communicator,
    drive_train_channel: Sender<DriveTrainCommand>,
}

impl ExternalComms {
    /// Instantiates the comms.
    /// This constructor will bind the listener.
    pub fn new(logging_channel: Sender<LogData>, sending_channel: Receiver<Box<SendableMessage>>,
               drive_train_channel: Sender<DriveTrainCommand>) -> ExternalComms {
        let communicator = Communicator::from(ADDRESS, PORT)
            .expect("Could not create communicator for external comms!");

        ExternalComms {
            sending_channel,
            logging_channel,
            communicator,
            drive_train_channel,
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
        if let Err(error) = self.communicator.check_connections() {
            match error {
                CommunicatorError::InvalidAddressError => panic!("Invalid address error for check_connections! This should not be possible"),
                CommunicatorError::DisconnectedListenerError => self.handle_lost_listener(),
            }
        }
    }

    fn send_messages(&mut self) {
        match self.sending_channel.try_recv() {
            Ok(message) => self.send_message(message),
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

        self.communicator.send_line(sending_string).expect("Error in sending a line!");
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