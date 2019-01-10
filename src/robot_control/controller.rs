use std::sync::mpsc::Receiver;

use crate::comms::CommsView;
use crate::logging::log_sender::LogSender;
use crate::robot_control::drive_train::DriveTrain;
use crate::robot_control::RobotControllerCommand;
use crate::framework::Runnable;

pub struct RobotController {
    log_view: LogSender,
    driver_station_view: CommsView,
    command_receiver: Receiver<RobotControllerCommand>,
    drive_train: DriveTrain,
}

impl Runnable for RobotController {
    fn init(&mut self) {
        unimplemented!()
    }

    fn run(&mut self) {
        unimplemented!()
    }
}