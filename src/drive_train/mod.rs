use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;

use crate::devices::motor_controllers::MotorController;
use crate::framework::logging::LogData;
use crate::framework::Runnable;

#[cfg(test)]
mod tests;

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
    left: Box<MotorController>,
    right: Box<MotorController>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl Subsystem<DriveTrainCommand> for DriveTrain
///////////////////////////////////////////////////////////////////////////////////////////////////
impl Runnable for DriveTrain {
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
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl DriveTrain
///////////////////////////////////////////////////////////////////////////////////////////////////
impl DriveTrain {
    /// Creates a new drive_train object which leverages the supplied channels for reporting errors
    /// and logging.
    pub fn new(command_receiver: Receiver<DriveTrainCommand>, log_channel: Sender<LogData>, left: Box<MotorController>, right: Box<MotorController>) -> DriveTrain {
        DriveTrain {
            is_enabled: true,
            is_alive: true,
            log_channel,
            command_receiver,
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
        // TODO
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