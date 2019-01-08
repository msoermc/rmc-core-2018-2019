use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;

use crate::comms::robot_communicator::CommsController;
use crate::comms::SendableMessage;
use crate::logging::log_data::LogData;
use crate::logging::log_sender::LogSender;
use crate::logging::LogAccepter;
use std::sync::RwLock;
use crate::framework::interfaces::TankDriveInterface;

pub mod factories;
mod commands;

pub enum SubsystemIdentifier {
    DriveTrainIdentifier,
}

impl ToString for SubsystemIdentifier {
    fn to_string(&self) -> String {
        match self {
            SubsystemIdentifier::DriveTrainIdentifier => "drive_train"
        }.to_string()
    }
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
    drive_interface: Box<TankDriveInterface>,
    log_sender: LogSender,
    message_sending_queue: Receiver<Box<SendableMessage>>,
    life_lock: Arc<RwLock<bool>>,
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
    fn get_drive_interface(&self) -> &Box<TankDriveInterface> {
        &self.drive_interface
    }

    fn kill(&self) {
        *self.life_lock.write().unwrap() = false;
    }

    fn revive(&self) {
        *self.life_lock.write().unwrap() = true;
    }
}

impl ConcreteDriverStationController {
    pub fn new(drive_interface: Box<TankDriveInterface>, log_sender: LogSender,
               message_sending_queue: Receiver<Box<SendableMessage>>, life_lock: Arc<RwLock<bool>>) -> Self
    {
        ConcreteDriverStationController {
            drive_interface,
            log_sender,
            message_sending_queue,
            life_lock,
        }
    }
}

pub trait DriverStationController: CommsController {
    fn get_drive_interface(&self) -> &Box<dyn TankDriveInterface>;
    fn kill(&self);
    fn revive(&self);
}