use std::sync::Arc;
use std::sync::mpsc::Sender;

use crate::status::life::GlobalLifeState;
use crate::status::robot_state::GlobalRobotState;

/// The controller module contains the `RobotController` struct.
/// The `RobotController` struct owns instances of the `DriveTrain` and the `MaterialHandler`.
pub mod controller;

/// The drive_train module contains the `DriveTrain` struct.
/// That structure is used to manage the physical drive train and perform operations on it.
pub mod drive_train;

pub mod material_handling;

#[derive(Clone, Debug, PartialEq)]
pub enum MechatronicsCommand {
    EnterDriveMode,
    EnterDumpMode,
    EnterDiggingMode,
    Drive(DriveCommandMessage),
    Brake,
    Dump,
    ResetDumper,
    StopDumper,
    Dig,
    StopDigging,
    RaiseActuators,
    LowerActuators,
    StopActuators,
}

pub struct MechatronicsMessageSender {
    channel: Sender<MechatronicsCommand>,
    state: Arc<GlobalRobotState>,
}

impl MechatronicsMessageSender {
    pub fn new(channel: Sender<MechatronicsCommand>, state: Arc<GlobalRobotState>) -> Self {
        Self {
            channel,
            state,
        }
    }

    pub fn revive(&self) {
        self.state.get_life().revive();
    }

    pub fn kill(&self) {
        self.brake();
        self.stop_digger();
        self.stop_dumper();
        self.stop_actuators();
        self.state.get_life().kill();
        self.brake();
        self.stop_digger();
        self.stop_dumper();
        self.stop_actuators();
    }

    /// Instructs the drive train to begin moving both sides at the provided speeds.
    /// Both speeds should be between -1 and 1.
    /// If any of the supplied speeds fall outside of that range, the command will not be sent and
    /// this method will return `Err(LogData)`.
    /// A negative speed indicates that the motors should be run in reverse.
    pub fn drive(&self, left: f32, right: f32) -> Result<(), ()> {
        let command = match DriveCommandMessage::create(left, right) {
            Ok(com) => com,
            Err(_) => return Err(()),
        };

        self.send_command(MechatronicsCommand::Drive(command));

        Ok(())
    }

    pub fn switch_to_drive(&self) {
        self.send_command(MechatronicsCommand::EnterDriveMode)
    }

    pub fn switch_to_dig(&self) {
        self.send_command(MechatronicsCommand::EnterDiggingMode)
    }

    pub fn switch_to_dump(&self) {
        self.send_command(MechatronicsCommand::EnterDumpMode)
    }

    /// Instructs the drive train to begin braking, halting all motion.
    pub fn brake(&self) {
        self.send_command(MechatronicsCommand::Brake)
    }

    pub fn dump(&self) {
        self.send_command(MechatronicsCommand::Dump)
    }

    pub fn reset_dumper(&self) {
        self.send_command(MechatronicsCommand::ResetDumper)
    }

    pub fn stop_dumper(&self) {
        self.send_command(MechatronicsCommand::StopDumper)
    }

    pub fn dig(&self) {
        self.send_command(MechatronicsCommand::Dig)
    }

    pub fn stop_digger(&self) {
        self.send_command(MechatronicsCommand::StopDigging)
    }

    pub fn raise_ladder(&self) {
        self.send_command(MechatronicsCommand::RaiseActuators)
    }

    pub fn lower_ladder(&self) {
        self.send_command(MechatronicsCommand::LowerActuators)
    }

    pub fn stop_actuators(&self) {
        self.send_command(MechatronicsCommand::StopActuators)
    }

    #[inline]
    fn send_command(&self, command: MechatronicsCommand) {
        self.channel.send(command).unwrap();
    }
}

/// Used by the `RobotControllerCommand` to represent a Drive command.
/// This is composed of the left and right speeds.
#[derive(Clone, Debug, PartialEq)]
pub struct DriveCommandMessage {
    left_speed: f32,
    right_speed: f32,
}

impl DriveCommandMessage {
    /// Constructs a drive command message, returning `Err(LogData)` if invalid arguments are given.
    pub fn create(left_speed: f32, right_speed: f32) -> Result<Self, ()> {
        if !(check_speed(left_speed) && check_speed(right_speed)) {
            warn!("Error in creating a DriveCommandMessage: left speed and right speed must be in range [-1, 1]!");
            Err(())
        } else {
            Ok(Self { left_speed, right_speed })
        }
    }

    pub fn get_left_speed(&self) -> f32 {
        self.left_speed
    }

    pub fn get_right_speed(&self) -> f32 {
        self.right_speed
    }
}

fn check_speed(speed: f32) -> bool {
    speed <= 1.0 && speed >= -1.0
}