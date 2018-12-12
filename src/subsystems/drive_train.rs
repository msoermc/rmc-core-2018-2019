use std::{
    io::Result,
    sync::{
        mpsc::{
            channel,
            Receiver,
            Sender,
        }
    },
};

use crate::{
    devices::{
        Device,
        motor_controllers::MotorController,
    },
    framework::{
        logging::LogData,
        Subsystem,
    },
};

pub enum DriveTrainEvent {
    RightSideMotorError(),
    LeftSideMotorError(),
}

pub enum DriveTrainCommand {
    Drive(f32, f32),
    Enable(),
    Disable(),
    Kill(),
    Revive(),
}

pub struct DriveTrain {
    is_enabled: bool,
    is_alive: bool,
    log_channel: Sender<LogData>,
    error_channel: Sender<DriveTrainEvent>,
    command_receiver: Receiver<DriveTrainCommand>,
    command_sender: Sender<DriveTrainCommand>,
    left: TankSide,
    right: TankSide,
}


impl Subsystem<DriveTrainCommand> for DriveTrain {
    fn init(&mut self) {
        // Do nothing
    }

    fn run(&mut self) {
        if let Ok(message) = self.command_receiver.recv() {
            match message {
                DriveTrainCommand::Drive(left, right) => {
                    if self.is_alive && self.is_enabled {
                        self.left.set_speed(left).unwrap();
                        self.right.set_speed(right).unwrap();
                    } else {
                        self.left.stop().unwrap();
                        self.right.stop().unwrap();
                    }
                }
                DriveTrainCommand::Enable() => {
                    self.is_enabled = true;
                }
                DriveTrainCommand::Disable() => {
                    self.is_enabled = false;
                }
                DriveTrainCommand::Kill() => {
                    self.is_alive = false;
                }
                DriveTrainCommand::Revive() => {
                    self.is_alive = true;
                }
            }
        }
    }

    fn get_command_sender(&mut self) -> Sender<DriveTrainCommand> {
        self.command_sender.clone()
    }
}


impl DriveTrain {
    pub fn new(log_channel: Sender<LogData>, error_channel: Sender<DriveTrainEvent>) -> DriveTrain {
        let (command_sender, command_receiver) = channel();
        DriveTrain {
            is_enabled: true,
            is_alive: true,
            log_channel,
            error_channel,
            command_receiver,
            command_sender,
            left: get_left_side(),
            right: get_right_side(),
        }
    }
}


struct TankSide {
    is_inverted: bool,
    front: Box<MotorController>,
    back: Box<MotorController>,
}


impl MotorController for TankSide {
    fn set_speed(&mut self, new_speed: f32) -> Result<()> {
        let potentially_inverted_speed = if self.is_inverted() {
            -new_speed
        } else {
            new_speed
        };

        self.front.set_speed(potentially_inverted_speed)?;
        self.back.set_speed(potentially_inverted_speed)
    }

    fn stop(&mut self) -> Result<()> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) {
        self.is_inverted = !self.is_inverted()
    }

    fn is_inverted(&self) -> bool {
        self.is_inverted
    }
}

fn get_left_side() -> TankSide {
    unimplemented!()
}

fn get_right_side() -> TankSide {
    unimplemented!()
}


impl Device for TankSide {}