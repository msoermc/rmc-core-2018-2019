use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread::spawn;

use crate::comms::Communicator;
use crate::comms::driver_station::parsing::*;
use crate::comms::SendableMessage;
use crate::framework::logging::LogData;
use crate::drive_train::DriveTrainCommand;

mod parsing;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 2401;


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
        let communicator = Communicator::from(ADDRESS, PORT);

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
            self.logging_channel.send(error).unwrap();
        }
    }

    fn send_messages(&mut self) {
        match self.sending_channel.try_recv() {
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
                Err(error) => self.logging_channel.send(error).unwrap()
            }
        }
    }

    fn handle_message(&mut self, message: &str) {
        let parsed_result = parse_message(message);
        match parsed_result {
            Ok(parsed_message) => self.handle_valid_command(parsed_message),
            Err(log) => self.logging_channel.send(log).unwrap(),
        }
    }

    fn send_message(&mut self, message: &SendableMessage) {
        let sending_string = message.encode();

        self.communicator.send_line(sending_string).expect("Error in sending a line!");
    }

    fn handle_sending_channel_disconnect(&mut self) {
        let log = LogData::fatal("Sending channel disconnected in external comms!");
        self.logging_channel.send(log)
            .expect("Sending channel and logging channel disconnected in Driver Station Comms!");
        panic!("{}", "Sending channel disconnected in external comms!");
    }

    fn handle_valid_command(&mut self, message: ReceivableMessage) {
        match message {
            ReceivableMessage::Kill => self.handle_kill_command(),
            ReceivableMessage::Revive => self.handle_revive_command(),
            ReceivableMessage::Enable(subsystem) => self.handle_enable_command(subsystem),
            ReceivableMessage::Disable(subsystem) => self.handle_disable_command(subsystem),
            ReceivableMessage::Drive(left_speed, right_speed) => self.handle_drive_command(left_speed, right_speed),
            ReceivableMessage::Brake => self.handle_brake_command(),
        }
    }

    fn handle_kill_command(&mut self) {
        self.drive_train_channel.send(DriveTrainCommand::Kill).unwrap();
    }

    fn handle_revive_command(&mut self) {
        self.drive_train_channel.send(DriveTrainCommand::Revive).unwrap();
    }

    fn handle_enable_command(&mut self, subsystem: ProtocolSubsystem) {
        match subsystem {
            ProtocolSubsystem::DriveTrain => self.drive_train_channel
                .send(DriveTrainCommand::Enable)
                .unwrap(),
        }
    }

    fn handle_disable_command(&mut self, subsystem: ProtocolSubsystem) {
        match subsystem {
            ProtocolSubsystem::DriveTrain => self.drive_train_channel
                .send(DriveTrainCommand::Disable)
                .unwrap(),
        }
    }

    fn handle_drive_command(&mut self, left_speed: f32, right_speed: f32) {
        self.drive_train_channel.send(DriveTrainCommand::Drive(left_speed, right_speed)).unwrap();
    }

    fn handle_brake_command(&mut self) {
        self.drive_train_channel.send(DriveTrainCommand::Stop).unwrap();
    }
}
