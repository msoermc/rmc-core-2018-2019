use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::RwLock;

use crate::comms::CommsView;
use crate::framework::Runnable;
use crate::logging::log_sender::LogSender;
use crate::logging::LogAccepter;
use crate::control::RobotControllerCommand;
use crate::control::drive_train::DriveTrain;
use crate::control::RobotLifeStatus;

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
            let res = match message {
                RobotControllerCommand::Drive(drive_command) => {
                    self.drive_train.drive(drive_command.left_speed, drive_command.right_speed)
                }
                RobotControllerCommand::Brake => {
                    self.drive_train.brake()
                }
                RobotControllerCommand::Enable => {
                    self.drive_train.enable();
                    Ok(())
                }
                RobotControllerCommand::Disable => {
                    self.drive_train.disable()
                }
            };

            if let Err(errors) = res {
                for error in errors {
                    self.log_view.accept_log(error.get_log())
                }
            }
        }

        if let Err(errors) = self.drive_train.run_cycle() {
            for error in errors {
                self.log_view.accept_log(error.get_log())
            }
        }
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
