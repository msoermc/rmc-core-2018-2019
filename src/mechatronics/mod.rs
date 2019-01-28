use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::RwLock;

/// The controller module contains the `RobotController` struct.
/// The `RobotController` struct owns instances of the `DriveTrain` and the `MaterialHandler`.
pub mod controller;

/// The drive_train module contains the `DriveTrain` struct.
/// That structure is used to manage the physical drive train and perform operations on it.
pub mod drive_train;

pub mod material_handling;

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

#[derive(Clone)]
pub struct GlobalLifeStatus {
    status: Arc<RwLock<RobotLifeStatus>>
}

impl GlobalLifeStatus {
    pub fn new() -> Self {
        Self {
            status: Arc::new(RwLock::new(RobotLifeStatus::Alive))
        }
    }

    pub fn get_status(&self) -> RobotLifeStatus {
        *self.status.read().unwrap()
    }

    pub fn is_alive(&self) -> bool {
        self.get_status() == RobotLifeStatus::Alive
    }

    pub fn kill(&self) {
        *self.status.write().unwrap() = RobotLifeStatus::Dead;
    }

    pub fn revive(&self) {
        *self.status.write().unwrap() = RobotLifeStatus::Alive;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MechatronicsCommand {
    Drive(DriveCommandMessage),
    Brake,
    Enable,
    Disable,
}

/// The `RobotView` struct is represents a view into the `RobotController`.
/// It is used to send requests to the controller to perform operations.
/// It is primarily used for inter thread messaging.
pub struct MechatronicsMessageSender {
    channel: Sender<MechatronicsCommand>,
    robot_life_status: GlobalLifeStatus,
}

impl MechatronicsMessageSender {
    /// Constructs a view, using a supplied `Sender` to send messages to the `RobotController`.
    /// The other end of the channel should be owned by the `RobotController`.
    pub fn new(channel: Sender<MechatronicsCommand>, robot_life_status: GlobalLifeStatus) -> Self {
        Self {
            channel,
            robot_life_status,
        }
    }

    /// Reenables the robot, allowing motor control.
    pub fn revive(&self) {
        self.robot_life_status.revive();
    }

    /// Disables the robot, preventing motor control.
    pub fn kill(&self) {
        self.brake();
        self.robot_life_status.kill();
        self.brake();
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

    /// Instructs the drive train to begin braking, halting all motion.
    pub fn brake(&self) {
        self.send_command(MechatronicsCommand::Brake)
    }

    /// Reenables the drive train, allowing motor control.
    pub fn enable_drive_train(&self) {
        self.send_command(MechatronicsCommand::Enable)
    }

    /// Disables the drive train, preventing motor control and causeing it to brake.
    pub fn disable_drive_train(&self)  {
        self.send_command(MechatronicsCommand::Disable)
    }

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