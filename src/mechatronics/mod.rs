use std::sync::mpsc::SyncSender;

use crate::mechatronics::commands::RobotCommand;

/// The controller module contains the `RobotController` struct.
/// The `RobotController` struct owns instances of the `DriveTrain` and the `MaterialHandler`.
pub mod controller;

pub mod commands;

/// The drive_train module contains the `DriveTrain` struct.
/// That structure is used to manage the physical drive train and perform operations on it.
pub mod drive_train;

pub mod dumper;

pub mod bucket_ladder;

pub struct RobotMessenger {
    channel: SyncSender<Box<RobotCommand>>,
}

impl RobotMessenger {
    pub fn new(channel: SyncSender<Box<RobotCommand>>) -> Self {
        Self {
            channel,
        }
    }

    #[inline]
    pub fn send_command(&self, command: Box<RobotCommand>) {
        self.channel.send(command).unwrap();
    }
}