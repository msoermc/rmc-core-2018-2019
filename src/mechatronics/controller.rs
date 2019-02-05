use std::sync::mpsc::Receiver;

use crate::framework::Runnable;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::material_handling::bucket_ladder::Ladder;
use crate::mechatronics::material_handling::dumper::Dumper;
use crate::mechatronics::MechatronicsCommand;
use crate::status::life::GlobalLifeStatus;

pub enum MechState {
    Digging,
    Driving,
    Dumping
}

pub struct RobotController {
    command_receiver: Receiver<MechatronicsCommand>,
    drive_train: DriveTrain,
    dumper: Dumper,
    digger: Ladder,
    life_status: GlobalLifeStatus,
    state: MechState,
}

impl Runnable for RobotController {
    fn init(&mut self) {
        info!("Initializing controller!");
        self.state = MechState::Driving;
        self.drive_train.enable();
        self.dumper.disable();
        self.digger.disable();
    }

    fn run(&mut self) {
        if let Ok(message) = self.command_receiver.try_recv() {
            self.handle_message(message);
        }

        self.process_states();
    }
}

impl RobotController {
    pub fn new(command_receiver: Receiver<MechatronicsCommand>,
               drive_train: DriveTrain, dumper: Dumper, ladder: Ladder, life_status: GlobalLifeStatus) -> Self {
        Self {
            command_receiver,
            drive_train,
            dumper,
            digger: ladder,
            life_status,
            state: MechState::Driving
        }
    }

    fn handle_message(&mut self, message: MechatronicsCommand) {
        match message {
            MechatronicsCommand::EnterDriveMode => {
                self.state = MechState::Driving;
                self.drive_train.enable();
                self.dumper.disable();
                self.digger.disable();
            },
            MechatronicsCommand::EnterDumpMode => {
                self.state = MechState::Dumping;
                self.dumper.enable();
                self.digger.disable();
                self.drive_train.disable();
            },
            MechatronicsCommand::EnterDiggingMode => {
                self.state = MechState::Digging;
                self.digger.enable();
                self.dumper.disable();
                self.drive_train.disable();
            },
            MechatronicsCommand::Drive(command) => {
                self.drive_train.drive(command.left_speed, command.right_speed);
            },
            MechatronicsCommand::Brake => {
                self.drive_train.brake();
            },
            MechatronicsCommand::Dump => {
                self.dumper.dump();
            },
            MechatronicsCommand::ResetDumper => {
                self.dumper.reset();
            },
            MechatronicsCommand::StopDumper => {
                self.dumper.stop();
            },
            MechatronicsCommand::Dig => {
                self.digger.dig()
            },
            MechatronicsCommand::StopDigging => {
                self.digger.stop_digging();
            },
            MechatronicsCommand::RaiseActuators => {
                self.digger.raise();
            },
            MechatronicsCommand::LowerActuators => {
                self.digger.lower();
            },
            MechatronicsCommand::StopActuators => {
                self.digger.stop_actuators();
            },
        }
    }

    fn process_states(&mut self) {
        // TODO implement
    }
}
