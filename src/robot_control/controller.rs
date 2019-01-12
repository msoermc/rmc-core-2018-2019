use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::RwLock;

use crate::comms::CommsView;
use crate::framework::Runnable;
use crate::logging::log_sender::LogSender;
use crate::robot_control::drive_train::DriveTrain;
use crate::robot_control::RobotControllerCommand;
use crate::robot_control::RobotLifeStatus;

pub struct RobotController {
    log_view: LogSender,
    driver_station_view: CommsView,
    command_receiver: Receiver<RobotControllerCommand>,
    drive_train: DriveTrain,
    life_status: Arc<RwLock<RobotLifeStatus>>,
}

impl Runnable for RobotController {
    fn init(&mut self) {
        // Do nothing
    }

    fn run(&mut self) {
        if let Ok(message) = self.command_receiver.try_recv() {
            match message {
                RobotControllerCommand::Drive(drive_command) => {
                    self.drive_train.drive(drive_command.left_speed, drive_command.right_speed);
                }
                RobotControllerCommand::Brake => {
                    self.drive_train.stop();
                }
                RobotControllerCommand::Enable => {
                    self.drive_train.enable();
                }
                RobotControllerCommand::Disable => {
                    self.drive_train.disable();
                }
            }
        }

        self.drive_train.run_cycle();
    }
}

impl RobotController {
    pub fn new(log_view: LogSender, driver_station_view: CommsView,
               command_receiver: Receiver<RobotControllerCommand>,
               drive_train: DriveTrain, life_status: Arc<RwLock<RobotLifeStatus>>) -> Self {
        Self {
            log_view,
            driver_station_view,
            command_receiver,
            drive_train,
            life_status,
        }
    }
}
