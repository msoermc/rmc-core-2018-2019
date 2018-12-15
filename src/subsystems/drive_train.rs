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

use chrono::prelude::Utc;

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

///////////////////////////////////////////////////////////////////////////////////////////////////
// enum DriveTrainEvent
///////////////////////////////////////////////////////////////////////////////////////////////////
/// The DriveTrainEvent enum has values representing different events that can occur on the
/// DriveTrain that are reported to the Controller thread.
pub enum DriveTrainEvent {}

///////////////////////////////////////////////////////////////////////////////////////////////////
// enum DriveTrainCommand
///////////////////////////////////////////////////////////////////////////////////////////////////
/// The DriveTrainCommand enum has values representing different commands that can be sent to the
/// DriveTrain over the command channel.
pub enum DriveTrainCommand {
    /// Drives both sides of the robot at their respective speeds.
    /// Speeds should be float values between -1 and 1.
    ///
    /// If the robot is currently in a dead state or the DriveTrain has been disabled, this command
    /// will cause the robot to brake instead.
    Drive(f32, f32),

    /// Commands the DriveTrain to begin braking.
    Stop,

    /// Enables the DriveTrain, allowing it to move if commanded to as normal.
    Enable,

    /// Disables the DriveTrain, causing it to halt it's motion.
    Disable,

    /// Informs the DriveTrain that the robot is now dead and that it should stop moving.
    Kill,

    /// Informs the subsystem that the robot is no longer dead and that the DriveTrain may resume
    /// normal operation.
    Revive,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// struct DriveTrain
///////////////////////////////////////////////////////////////////////////////////////////////////
/// The DriveTrain struct contains the data required to run the DriveTrain. The DriveTrain
/// is normally run in it's own thread and communication with it is done via channels.
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

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl Subsystem<DriveTrainCommand> for DriveTrain
///////////////////////////////////////////////////////////////////////////////////////////////////
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
                DriveTrainCommand::Enable => {
                    self.enable();
                }
                DriveTrainCommand::Disable => {
                    self.disable();
                }
                DriveTrainCommand::Kill => {
                    self.kill();
                }
                DriveTrainCommand::Revive => {
                    self.revive();
                }
                DriveTrainCommand::Stop => {
                    self.stop();
                }
            },
            Err(e) => {
                if let TryRecvError::Disconnected = e {
                    let error_log = LogData::new(LogType::Fatal, Utc::now(), e.to_string());
                    self.log_channel.send(error_log).unwrap(); // Nothing we can do here, we are fucked
                }
            }
        }
    }

    fn get_command_sender(&mut self) -> Sender<DriveTrainCommand> {
        self.command_sender.clone()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl DriveTrain
///////////////////////////////////////////////////////////////////////////////////////////////////
impl DriveTrain {
    /// Creates a new drive_train object which leverages the supplied channels for reporting errors
    /// and logging.
    pub fn new(log_channel: Sender<LogData>, error_channel: Sender<DriveTrainEvent>) -> DriveTrain {
        let (command_sender, command_receiver) = channel();
        DriveTrain {
            is_enabled: true,
            is_alive: true,
            log_channel: log_channel.clone(),
            error_channel: error_channel.clone(),
            command_receiver,
            command_sender,
            left: get_left_side(log_channel.clone(), error_channel.clone()),
            right: get_right_side(log_channel, error_channel),
        }
    }

    /// Causes the DriveTrain to drive at the supplied speeds.
    /// If the subsystem is disabled or the robot has been killed, this method will instead cause
    /// the robot to brake.
    fn drive(&mut self, left_speed: f32, right_speed: f32) {
        if self.is_alive && self.is_enabled {
            self.left.set_speed(left_speed).unwrap(); // TODO handle errors
            self.right.set_speed(right_speed).unwrap(); // TODO handle errors
        } else {
            self.stop();
        }
    }

    /// Causes the DriveTrain to brake.
    fn stop(&mut self) {
        self.left.stop().unwrap(); // TODO handle errors
        self.right.stop().unwrap(); // TODO handle errors
    }

    /// Enables the DriveTrain/
    fn enable(&mut self) {
        self.is_enabled = true;
    }

    /// Causes the DriveTrain to brake and disables it.
    fn disable(&mut self) {
        self.is_enabled = false;
        self.stop();
    }

    /// Causes the DriveTrain to brake and informs it that the robot has been killed.
    fn kill(&mut self) {
        self.is_alive = false;
        self.stop();
    }

    /// Informs the DriveTrain that the robot is alive.
    fn revive(&mut self) {
        self.is_alive = true;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// struct TankSide
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Represents one side of the drive train. This structure serves as an abstraction, allowing the
/// rest of the subsystem to function the same regardless of the amount of motors on each side.
struct TankSide {
    is_inverted: bool,
    front: Box<MotorController<HoverBoardError>>,
    back: Box<MotorController<HoverBoardError>>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl MotorController<TankSideError> for TankSide
///////////////////////////////////////////////////////////////////////////////////////////////////
impl MotorController<TankSideError> for TankSide {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), TankSideError> {
        if let Err(_) = self.front.set_speed(new_speed) {
            unimplemented!()
        };
        if let Err(_) = self.back.set_speed(new_speed) {
            unimplemented!()
        };

        Ok(())
    }

    fn stop(&mut self) -> Result<(), TankSideError> {
        self.set_speed(0.0)
    }

    fn invert(&mut self) -> Result<(), TankSideError> {
        self.is_inverted = !self.is_inverted();
        if let Err(_) = self.front.invert() {
            unimplemented!()
        };
        if let Err(_) = self.back.invert() {
            unimplemented!()
        };

        Ok(())
    }

    fn is_inverted(&self) -> bool {
        self.is_inverted
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// enum TankSideError
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The TankSideError enum serves as an abstraction over the various physical errors which can
/// occur with different hardware configurations.
enum TankSideError {}

///////////////////////////////////////////////////////////////////////////////////////////////////
// fn get_left_side
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Factory function to generate the left side of the DriveTrain.
fn get_left_side(log_channel: Sender<LogData>, error_channel: Sender<DriveTrainEvent>) -> TankSide {
    unimplemented!()
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// fn get_right_side
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Factory function to generate the right side of the DriveTrain.
fn get_right_side(log_channel: Sender<LogData>, error_channel: Sender<DriveTrainEvent>) -> TankSide {
    unimplemented!()
}