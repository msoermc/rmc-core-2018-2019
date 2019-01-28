use std::sync::mpsc::Receiver;

use crate::comms::ServerSender;
use crate::framework::Runnable;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::GlobalLifeStatus;
use crate::mechatronics::material_handling::bucket_ladder::BucketLadder;
use crate::mechatronics::material_handling::dumper::Dumper;
use crate::mechatronics::MechatronicsCommand;

pub struct RobotController {
    driver_station_view: ServerSender,
    command_receiver: Receiver<MechatronicsCommand>,
    drive_train: DriveTrain,
    dumper: Dumper,
    ladder: BucketLadder,
    life_status: GlobalLifeStatus,
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
               drive_train: DriveTrain, dumper: Dumper, ladder: BucketLadder, life_status: GlobalLifeStatus) -> Self {
        Self {
            driver_station_view,
            command_receiver,
            drive_train,
            dumper,
            ladder,
            life_status,
        }
    }

    fn handle_message(&mut self, message: MechatronicsCommand) {
        match message {
            MechatronicsCommand::Drive(drive_command) => {
                self.drive_train.drive(drive_command.left_speed, drive_command.right_speed);
            }
            MechatronicsCommand::Brake => {
                self.drive_train.brake();
            }
            MechatronicsCommand::EnableDrive => {
                self.drive_train.enable();
            }
            MechatronicsCommand::DisableDrive => {
                self.drive_train.disable();
            }
            MechatronicsCommand::EnableDumper => {
                self.dumper.enable();
            }
            MechatronicsCommand::DisableDumper => {
                self.dumper.disable();
            }
            MechatronicsCommand::EnableBucketLadder => {
                self.ladder.enable();
            }
            MechatronicsCommand::DisableBucketLadder => {
                self.ladder.disable();
            }
            MechatronicsCommand::Dump => {
                self.dumper.dump();
            }
            MechatronicsCommand::ResetDumper => {
                self.dumper.reset();
            }
            MechatronicsCommand::StopDumper => {
                self.dumper.stop();
            }
            MechatronicsCommand::Dig => {
                self.ladder.dig();
            }
            MechatronicsCommand::StopDigging => {
                self.ladder.stop_digging();
            }
            MechatronicsCommand::RaiseDigger => {
                self.ladder.raise();
            }
            MechatronicsCommand::LowerDigger => {
                self.ladder.lower();
            }
            MechatronicsCommand::FreezeDiggerHeight => {
                self.ladder.stop_actuators();
            }
        }
    }

    fn process_states(&mut self) {
        // TODO implement
    }
}
