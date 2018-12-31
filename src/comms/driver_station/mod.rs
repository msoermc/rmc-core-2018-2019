use crate::comms::command_io_controller::RobotInterface;
use crate::comms::SendableMessage;
use crate::logging::LogAccepter;
use crate::logging::log_data::LogData;
use crate::drive_train::DriveTrainCommand;
use std::sync::mpsc::Sender;
use crate::logging::log_sender::LogSender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;

mod parsers;
mod commands;

enum SubsystemIdentifier {
    DriveTrainIdentifier,
}

struct DriverStationInterface {
    drive_channel: Sender<DriveTrainCommand>,
    log_sender: LogSender,
    message_sending_queue: Receiver<Box<SendableMessage>>,
}

impl RobotInterface for DriverStationInterface {
    fn get_next_requested_send(&self) -> Option<Box<SendableMessage>> {
        match self.message_sending_queue.try_recv() {
            Ok(message) => Some(message),
            Err(TryRecvError::Disconnected) => panic!("Comms sending queue was disconnected!"),
            Err(TryRecvError::Empty) => None  // Do nothing
        }
    }
}

impl LogAccepter for DriverStationInterface {
    fn accept_log(&mut self, log: LogData) {
        self.log_sender.accept_log(log)
    }
}

impl DriverStationInterface {
    pub fn send_drive_train_command(&self, command: DriveTrainCommand) {
        self.drive_channel.send(command).unwrap();
    }
}