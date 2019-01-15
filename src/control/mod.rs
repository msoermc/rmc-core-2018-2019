use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::RwLock;

use crate::logging::log_data::LogData;

/// The controller module contains the `RobotController` struct.
/// The `RobotController` struct owns instances of the `DriveTrain` and the `MaterialHandler`.
pub mod controller;

/// The drive_train module contains the `DriveTrain` struct.
/// That structure is used to manage the physical drive train and perform operations on it.
pub mod drive_train;

/// Represents the current status of the robot.
/// Many subsystems will check this before determining if it is safe to perform an operation.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RobotLifeStatus {
    /// Indicates that the robot is in a normal operating state.
    Alive,

    /// Indicates that the robot has been disabled by the operators and that it is not
    /// safe to perform many operations.
    Dead,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RobotControllerCommand {
    Drive(DriveCommandMessage),
    Brake,
    Enable,
    Disable,
}

/// The `RobotView` struct is represents a view into the `RobotController`.
/// It is used to send requests to the controller to perform operations.
/// It is primarily used for inter thread messaging.
pub struct RobotView {
    channel: Sender<RobotControllerCommand>,
    robot_life_status: Arc<RwLock<RobotLifeStatus>>,
}

impl RobotView {
    /// Constructs a view, using a supplied `Sender` to send messages to the `RobotController`.
    /// The other end of the channel should be owned by the `RobotController`.
    pub fn new(channel: Sender<RobotControllerCommand>, robot_life_status: Arc<RwLock<RobotLifeStatus>>) -> Self {
        Self {
            channel,
            robot_life_status,
        }
    }

    /// Reenables the robot, allowing motor control.
    pub fn revive(&self) -> Result<(), LogData> {
        self.change_life_status(RobotLifeStatus::Alive)
    }

    /// Disables the robot, preventing motor control.
    pub fn kill(&self) -> Result<(), LogData> {
        self.change_life_status(RobotLifeStatus::Dead)
    }

    /// Instructs the drive train to begin moving both sides at the provided speeds.
    /// Both speeds should be between -1 and 1.
    /// If any of the supplied speeds fall outside of that range, the command will not be sent and
    /// this method will return `Err(LogData)`.
    /// A negative speed indicates that the motors should be run in reverse.
    pub fn drive(&self, left: f32, right: f32) -> Result<(), LogData> {
        let command = DriveCommandMessage::create(left, right)?;

        self.send_command(RobotControllerCommand::Drive(command))
    }

    /// Instructs the drive train to begin braking, halting all motion.
    pub fn brake(&self) -> Result<(), LogData> {
        self.send_command(RobotControllerCommand::Brake)
    }

    /// Reenables the drive train, allowing motor control.
    pub fn enable_drive_train(&self) -> Result<(), LogData> {
        self.send_command(RobotControllerCommand::Enable)
    }

    /// Disables the drive train, preventing motor control and causeing it to brake.
    pub fn disable_drive_train(&self) -> Result<(), LogData> {
        self.send_command(RobotControllerCommand::Disable)
    }

    fn send_command(&self, command: RobotControllerCommand) -> Result<(), LogData> {
        match self.channel.send(command) {
            Ok(_) => Ok(()),
            Err(_) => Err(LogData::fatal("Failed to send message to RobotController!")),
        }
    }

    fn change_life_status(&self, status: RobotLifeStatus) -> Result<(), LogData> {
        match self.robot_life_status.write() {
            Ok(mut flag) => {
                *flag = status;
                Ok(())
            }
            Err(_) => Err(LogData::fatal("Failed to revive robot!")),
        }
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
    fn create(left_speed: f32, right_speed: f32) -> Result<Self, LogData> {
        if !(check_speed(left_speed) && check_speed(right_speed)) {
            Err(LogData::warning("Error in creating a DriveCommandMessage: left speed and right speed must be in range [-1, 1]!"))
        } else {
            Ok(Self { left_speed, right_speed })
        }
    }
}

fn check_speed(speed: f32) -> bool {
    speed <= 1.0 && speed >= -1.0
}