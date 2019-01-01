use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;

use crate::comms::robot_communicator::CommsController;
use crate::comms::SendableMessage;
use crate::drive_train::DriveTrainCommand;
use crate::logging::log_data::LogData;
use crate::logging::log_sender::LogSender;
use crate::logging::LogAccepter;

pub mod factories;
mod commands;

pub enum SubsystemIdentifier {
    DriveTrainIdentifier,
}

impl FromStr for SubsystemIdentifier {
    type Err = LogData;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "drive_train" => Ok(SubsystemIdentifier::DriveTrainIdentifier),
            _ => Err(LogData::error("Unparseable SubsystemIdentifier!"))
        }
    }
}

pub struct ConcreteDriverStationController {
    drive_channel: Sender<DriveTrainCommand>,
    log_sender: LogSender,
    message_sending_queue: Receiver<Box<SendableMessage>>,
}

impl CommsController for ConcreteDriverStationController {
    fn get_next_requested_send(&self) -> Option<Box<SendableMessage>> {
        match self.message_sending_queue.try_recv() {
            Ok(message) => Some(message),
            Err(TryRecvError::Disconnected) => panic!("Comms sending queue was disconnected!"),
            Err(TryRecvError::Empty) => None  // Do nothing
        }
    }
}

impl LogAccepter for ConcreteDriverStationController {
    fn accept_log(&mut self, log: LogData) {
        self.log_sender.accept_log(log)
    }
}

impl DriverStationController for ConcreteDriverStationController {
    fn send_drive_train_command(&self, command: DriveTrainCommand) {
        self.drive_channel.send(command).unwrap();
    }
}

impl ConcreteDriverStationController {
    pub fn new(drive_channel: Sender<DriveTrainCommand>, log_sender: LogSender,
               message_sending_queue: Receiver<Box<SendableMessage>>) -> Self
    {
        ConcreteDriverStationController {
            drive_channel,
            log_sender,
            message_sending_queue,
        }
    }
}

pub trait DriverStationController: CommsController {
    fn send_drive_train_command(&self, command: DriveTrainCommand);
}