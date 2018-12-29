use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread::spawn;

use crate::comms::Communicator;
use crate::comms::driver_station::parsing::*;
use crate::comms::SendableMessage;
use crate::drive_train::DriveTrainCommand;
use crate::logging::log_data::LogData;
use crate::logging::log_sender::LogSender;
use crate::logging::LogAccepter;

mod parsing;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 2401;


pub struct DriverStationComms {
    message_queue: Receiver<Box<SendableMessage>>,
    log_sender: LogSender,
    communicator: Communicator,
    drive_train_channel: Sender<DriveTrainCommand>,
}

impl DriverStationComms {
    /// Instantiates the comms.
    /// This constructor will bind the listener.
    pub fn new(logging_channel: LogSender, message_queue: Receiver<Box<SendableMessage>>,
               drive_train_channel: Sender<DriveTrainCommand>) -> DriverStationComms {
        let communicator = Communicator::from(ADDRESS, PORT);

        DriverStationComms {
            message_queue,
            log_sender: logging_channel,
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
        let parsed_result = parse_message(message);
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
        self.drive_train_channel.send(DriveTrainCommand::Kill);
    }

    fn handle_revive_command(&mut self) {
        self.drive_train_channel.send(DriveTrainCommand::Revive);
    }

    fn handle_enable_command(&mut self, subsystem: ProtocolSubsystem) {
        match subsystem {
            ProtocolSubsystem::DriveTrain => self.drive_train_channel
                .send(DriveTrainCommand::Enable),
        };
    }

    fn handle_disable_command(&mut self, subsystem: ProtocolSubsystem) {
        match subsystem {
            ProtocolSubsystem::DriveTrain => self.drive_train_channel
                .send(DriveTrainCommand::Disable),
        };
    }

    fn handle_drive_command(&mut self, left_speed: f32, right_speed: f32) {
        self.drive_train_channel.send(DriveTrainCommand::Drive(left_speed, right_speed));
    }

    fn handle_brake_command(&mut self) {
        self.drive_train_channel.send(DriveTrainCommand::Stop);
    }
}
