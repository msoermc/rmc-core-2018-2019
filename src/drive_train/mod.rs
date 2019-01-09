use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::sync::RwLock;

use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::MotorFailure;
use crate::framework::Runnable;
use crate::logging::log_sender::LogSender;
use crate::logging::LogAccepter;

pub mod interface;

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

/// The DriveTrain struct contains the data required to run the DriveTrain. The DriveTrain
/// is normally run in it's own thread and communication with it is done via channels.
pub struct DriveTrain {
    is_enabled: bool,
    left: MotorGroup,
    right: MotorGroup,
    is_alive: Arc<RwLock<bool>>,
}

impl DriveTrain {
    pub fn new(left: MotorGroup, right: MotorGroup, robot_life: Arc<RwLock<bool>>) -> DriveTrain {
        DriveTrain {
            is_enabled: true,
            left,
            right,
            is_alive: robot_life,
        }
    }

    pub fn run_cycle(&mut self) -> Result<(), Vec<MotorFailure>> {
        let mut errors = Vec::new();

        if self.is_enabled && *self.is_alive.read().expect("Drive train failed to read life") {
            if let Err(e) = &mut self.maintain_last() {
                errors.append(e);
            }
        } else {
            if let Err(e) = &mut self.stop() {
                errors.append(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn drive(&mut self, left_speed: f32, right_speed: f32) -> Result<(), Vec<MotorFailure>> {
        unimplemented!()
    }

    pub fn stop(&mut self) -> Result<(), Vec<MotorFailure>> {
        unimplemented!()
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) -> Result<(), Vec<MotorFailure>> {
        self.is_enabled = false;
        self.stop()
    }

    fn maintain_last(&mut self) -> Result<(), Vec<MotorFailure>> {
        unimplemented!()
    }
}