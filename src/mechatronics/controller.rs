use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::RwLock;

use crate::comms::ServerSender;
use crate::framework::Runnable;
use crate::mechatronics::MechatronicsCommand;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::RobotLifeStatus;

pub struct RobotController {
    driver_station_view: ServerSender,
    command_receiver: Receiver<MechatronicsCommand>,
    drive_train: DriveTrain,
    life_status: Arc<RwLock<RobotLifeStatus>>,
}

impl Runnable for RobotController {
    fn init(&mut self) {
        info!("Initializing controller!");
    }

    fn run(&mut self) {
        if let Ok(message) = self.command_receiver.try_recv() {
            self.handle_message(message);
        }

        self.process_states();
    }
}

impl RobotController {
    pub fn new(driver_station_view: ServerSender,
               command_receiver: Receiver<MechatronicsCommand>,
               drive_train: DriveTrain, life_status: Arc<RwLock<RobotLifeStatus>>) -> Self {
        Self {
            driver_station_view,
            command_receiver,
            drive_train,
            life_status,
        }
    }

    fn handle_message(&mut self, message: MechatronicsCommand) {
        match message {
            MechatronicsCommand::Drive(drive_command) => {
                self.drive_train.drive(drive_command.left_speed, drive_command.right_speed)
            }
            MechatronicsCommand::Brake => {
                self.drive_train.brake()
            }
            MechatronicsCommand::Enable => {
                self.drive_train.enable();
            }
            MechatronicsCommand::Disable => {
                self.drive_train.disable()
            }
        }
    }

    fn process_states(&mut self) {
        // TODO implement
    }
}
