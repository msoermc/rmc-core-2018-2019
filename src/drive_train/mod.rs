use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::sync::RwLock;

use crate::devices::motor_controllers::MotorController;
use crate::framework::Runnable;
use crate::logging::log_sender::LogSender;
use crate::logging::LogAccepter;
use crate::devices::motor_controllers::motor_group::MotorGroup;

pub mod interface;

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
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// struct DriveTrain
///////////////////////////////////////////////////////////////////////////////////////////////////
/// The DriveTrain struct contains the data required to run the DriveTrain. The DriveTrain
/// is normally run in it's own thread and communication with it is done via channels.
pub struct DriveTrain {
    is_enabled: bool,
    log_sender: LogSender,
    command_receiver: Receiver<DriveTrainCommand>,
    left: MotorGroup,
    right: MotorGroup,
    is_alive: Arc<RwLock<bool>>,
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
            Ok(command) => {
                self.handle_new_command(command);
            }
            Err(TryRecvError::Disconnected) => {
                self.handle_command_channel_disconnect();
            }
            Err(TryRecvError::Empty) => ()
        }

        if !*self.is_alive.read().unwrap() {
            self.kill();
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl DriveTrain
///////////////////////////////////////////////////////////////////////////////////////////////////
impl DriveTrain {
    /// Creates a new drive_train object which leverages the supplied channels for reporting errors
    /// and logging.
    pub fn new(command_receiver: Receiver<DriveTrainCommand>, log_sender: LogSender, left: MotorGroup, right: MotorGroup, robot_life: Arc<RwLock<bool>>) -> DriveTrain {
        DriveTrain {
            is_enabled: true,
            log_sender,
            command_receiver,
            left,
            right,
            is_alive: robot_life,
        }
    }

    /// Respond to a new command that has been received using the appropriate method.
    fn handle_new_command(&mut self, command: DriveTrainCommand) {
        match command {
            DriveTrainCommand::Drive(left, right) => self.drive(left, right),
            DriveTrainCommand::Enable => self.enable(),
            DriveTrainCommand::Disable => self.disable(),
            DriveTrainCommand::Stop => self.stop(),
        }
    }

    fn handle_command_channel_disconnect(&mut self) {
        let description = "DriveTrain command channel disconnected!";
        self.log_sender.send_fatal(description);
    }

    /// Causes the DriveTrain to drive at the supplied speeds.
    /// If the subsystem is disabled or the robot has been killed, this method will instead cause
    /// the robot to brake.
    fn drive(&mut self, left_speed: f32, right_speed: f32) {
        if *self.is_alive.read().unwrap() && self.is_enabled {
            if let Err(e) = self.left.set_speed(left_speed) {
                self.log_sender.accept_log(e);
            }

            if let Err(e) = self.right.set_speed(right_speed) {
                self.log_sender.accept_log(e);
            }
        } else {
            self.stop();
        }
    }

    fn stop(&mut self) {
        if let Err(e) = self.left.stop() {
            self.log_sender.accept_log(e);
        }

        if let Err(e) = self.right.stop() {
            self.log_sender.accept_log(e);
        }
    }

    fn enable(&mut self) {
        self.is_enabled = true;
    }

    fn disable(&mut self) {
        self.is_enabled = false;
        self.stop();
    }

    fn kill(&mut self) {
        self.stop();
    }
}