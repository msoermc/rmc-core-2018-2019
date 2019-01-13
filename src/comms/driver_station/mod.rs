use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;

use crate::comms::CommsController;
use crate::comms::SendableMessage;
use crate::logging::log_data::LogData;
use crate::logging::log_sender::LogSender;
use crate::logging::LogAccepter;
use crate::robot_control::RobotView;

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
    view: RobotView,
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
    fn get_view(&self) -> &RobotView {
        &self.view
    }
}

impl ConcreteDriverStationController {
    pub fn new(view: RobotView, log_sender: LogSender, message_sending_queue: Receiver<Box<SendableMessage>>) -> Self
    {
        ConcreteDriverStationController {
            view,
            log_sender,
            message_sending_queue,
        }
    }
}

pub trait DriverStationController: CommsController {
    fn get_view(&self) -> &RobotView;
}