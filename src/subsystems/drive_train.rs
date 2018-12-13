use std::{
    sync::{
        mpsc::{
            channel,
            Receiver,
            Sender,
            TryRecvError,
        }
    }
};

use crate::{
    devices::{
        motor_controllers::{
            MotorController,
            hover_board::HoverBoardError,
        }
    },
    framework::{
        logging::{
            LogData,
            LogType,
        },
        Subsystem,
    },
};

use chrono::prelude::Utc;

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
        match self.command_receiver.try_recv() {
            Ok(message) => match message {
                DriveTrainCommand::Drive(left, right) => {
                    self.drive(left, right);
                }
                DriveTrainCommand::Enable() => {
                    self.enable();
                }
                DriveTrainCommand::Disable() => {
                    self.disable();
                }
                DriveTrainCommand::Kill() => {
                    self.kill();
                }
                DriveTrainCommand::Revive() => {
                    self.revive();
                }
            },
            Err(e) => {
                if let TryRecvError::Disconnected = e {
                    let error_log = LogData::new(LogType::Fatal(), Utc::now(), e.to_string());
                    self.log_channel.send(error_log).unwrap(); // Nothing we can do here, we are fucked
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

    fn drive(&mut self, left_speed: f32, right_speed: f32) {
        if self.is_alive && self.is_enabled {
            self.left.set_speed(left_speed).unwrap();
            self.right.set_speed(right_speed).unwrap();
        } else {
            self.stop();
        }
    }

    fn stop(&mut self) {
        self.left.stop().unwrap();
        self.right.stop().unwrap();
    }

    fn enable(&mut self) {
        self.is_enabled = true;
    }

    fn disable(&mut self) {
        self.is_enabled = false;
        self.stop();
    }

    fn kill(&mut self) {
        self.is_alive = false;
        self.stop();
    }

    fn revive(&mut self) {
        self.is_alive = true;
    }
}


struct TankSide {
    is_inverted: bool,
    front: Box<MotorController<HoverBoardError>>,
    back: Box<MotorController<HoverBoardError>>,
}


impl MotorController<TankSideError> for TankSide {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), TankSideError> {
        self.front.set_speed(new_speed).unwrap();
        self.back.set_speed(new_speed).unwrap();

        Ok(())
    }

    fn stop(&mut self) -> Result<(), TankSideError> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) -> Result<(), TankSideError> {
        self.is_inverted = !self.is_inverted();
        self.front.invert().unwrap();
        self.back.invert().unwrap();
        Ok(())
    }

    fn is_inverted(&self) -> bool {
        self.is_inverted
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TankSideError {}

fn get_left_side() -> TankSide {
    unimplemented!()
}

fn get_right_side() -> TankSide {
    unimplemented!()
}