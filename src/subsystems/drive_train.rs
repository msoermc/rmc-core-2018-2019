use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;

use crate::devices::motor_controllers::MotorController;
use crate::framework::logging::get_timestamp;
use crate::framework::logging::LogData;
use crate::framework::logging::LogType;
use crate::framework::Subsystem;

///////////////////////////////////////////////////////////////////////////////////////////////////
// enum DriveTrainCommand
///////////////////////////////////////////////////////////////////////////////////////////////////
/// The DriveTrainCommand enum has values representing different commands that can be sent to the
/// DriveTrain over the command channel.
#[derive(Copy, Clone, Debug, PartialEq)]
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
    command_receiver: Receiver<DriveTrainCommand>,
    command_sender: Sender<DriveTrainCommand>,
    left: Box<MotorController>,
    right: Box<MotorController>,
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
            Ok(command) => self.handle_new_command(command),
            Err(TryRecvError::Disconnected) => {
                self.handle_command_channel_disconnect();
            }
            Err(TryRecvError::Empty) => ()
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
    pub fn new(log_channel: Sender<LogData>, left: Box<MotorController>, right: Box<MotorController>) -> DriveTrain {
        let (command_sender, command_receiver) = channel();
        DriveTrain {
            is_enabled: true,
            is_alive: true,
            log_channel,
            command_receiver,
            command_sender,
            left,
            right,
        }
    }

    /// Respond to a new command that has been received using the appropriate method.
    fn handle_new_command(&mut self, command: DriveTrainCommand) {
        match command {
            DriveTrainCommand::Drive(left, right) => self.drive(left, right),
            DriveTrainCommand::Enable => self.enable(),
            DriveTrainCommand::Disable => self.disable(),
            DriveTrainCommand::Kill => self.kill(),
            DriveTrainCommand::Revive => self.revive(),
            DriveTrainCommand::Stop => self.stop(),
        }
    }

    fn handle_command_channel_disconnect(&mut self) {
        let severity = LogType::Fatal;
        let timestamp = get_timestamp();
        let description = "DriveTrain: Command channel was disconnected!".to_string();
        let error_log = LogData::new(severity, timestamp, description);
        self.log_channel.send(error_log).unwrap(); // Fail-fast if logger dies
    }

    /// Causes the DriveTrain to drive at the supplied speeds.
    /// If the subsystem is disabled or the robot has been killed, this method will instead cause
    /// the robot to brake.
    fn drive(&mut self, left_speed: f32, right_speed: f32) {
        if self.is_alive && self.is_enabled {
            self.left.set_speed(left_speed);
            self.right.set_speed(right_speed);
        } else {
            self.stop();
        }
    }

    fn stop(&mut self) {
        self.left.stop();
        self.right.stop();
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

///////////////////////////////////////////////////////////////////////////////////////////////////
// struct TankSide
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Represents one side of the drive train. This structure serves as an abstraction, allowing the
/// rest of the subsystem to function the same regardless of the amount of motors on each side.
pub struct TankSide {
    is_inverted: bool,
    motors: Vec<Box<MotorController>>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl MotorController for TankSide
///////////////////////////////////////////////////////////////////////////////////////////////////
impl MotorController for TankSide {
    fn set_speed(&mut self, new_speed: f32) {
        for motor in &mut self.motors {
            motor.set_speed(new_speed);
        }
    }

    fn stop(&mut self) {
        self.set_speed(0.0)
    }

    fn invert(&mut self) {
        self.is_inverted = !self.is_inverted();

        for motor in &mut self.motors {
            motor.invert();
        }
    }

    fn is_inverted(&self) -> bool {
        self.is_inverted
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl TankSide
///////////////////////////////////////////////////////////////////////////////////////////////////
impl TankSide {
    pub fn new(motors: Vec<Box<MotorController>>) -> TankSide {
        TankSide {
            is_inverted: false,
            motors,
        }
    }
}