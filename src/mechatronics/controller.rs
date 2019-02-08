use std::sync::Arc;
use std::sync::mpsc::Receiver;

use crate::framework::Runnable;
use crate::mechatronics::bucket_ladder::Intake;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::dumper::Dumper;
use crate::mechatronics::MechatronicsCommand;
use crate::status::life::GlobalLifeState;

pub enum MechState {
    Digging,
    Driving,
    Dumping,
}

pub struct RobotController {
    command_receiver: Receiver<MechatronicsCommand>,
    drive_train: DriveTrain,
    dumper: Dumper,
    intake: Intake,
    life: Arc<GlobalLifeState>,
    state: MechState,
}

impl Runnable for RobotController {
    fn init(&mut self) {
        info!("Initializing controller!");
        self.state = MechState::Driving;
        self.drive_train.enable();
        self.dumper.disable();
        self.intake.disable();
    }

    fn run(&mut self) {
        if let Ok(message) = self.command_receiver.try_recv() {
            self.handle_message(message);
        }

        self.drive_train.run_cycle();
        self.dumper.run_cycle();
        self.intake.run_cycle();
    }
}

impl RobotController {
    pub fn new(command_receiver: Receiver<MechatronicsCommand>,
               drive_train: DriveTrain, dumper: Dumper, intake: Intake, life: Arc<GlobalLifeState>) -> Self {
        Self {
            command_receiver,
            drive_train,
            dumper,
            intake,
            life,
            state: MechState::Driving,
        }
    }

    fn handle_message(&mut self, message: MechatronicsCommand) {
        match message {
            MechatronicsCommand::EnterDriveMode => {
                self.state = MechState::Driving;
                self.drive_train.enable();
                self.dumper.disable();
                self.intake.disable();
            }
            MechatronicsCommand::EnterDumpMode => {
                self.state = MechState::Dumping;
                self.dumper.enable();
                self.intake.disable();
                self.drive_train.disable();
            }
            MechatronicsCommand::EnterDiggingMode => {
                self.state = MechState::Digging;
                self.intake.enable();
                self.dumper.disable();
                self.drive_train.disable();
            }
            MechatronicsCommand::Drive(command) => {
                self.drive_train.drive(command.left_speed, command.right_speed);
            }
            MechatronicsCommand::Brake => {
                self.drive_train.brake();
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
                self.intake.dig()
            }
            MechatronicsCommand::StopDigging => {
                self.intake.stop_ladder();
            }
            MechatronicsCommand::RaiseActuators => {
                self.intake.raise();
            }
            MechatronicsCommand::LowerActuators => {
                self.intake.lower();
            }
            MechatronicsCommand::StopActuators => {
                self.intake.stop_actuators();
            }
        }
    }
}
