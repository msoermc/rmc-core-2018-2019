use std::sync::Arc;

use crate::mechatronics::drive_train::state::GlobalDriveTrainState;
use crate::motor_controllers::MotorController;
use crate::status::life::GlobalLifeState;

pub mod state;

#[cfg(test)]
mod tests;

/// Manages and controls the drive train.
pub struct DriveTrain {
    state: Arc<GlobalDriveTrainState>,
    left: Box<MotorController>,
    right: Box<MotorController>,
    robot_status: Arc<GlobalLifeState>,
    enabled: bool,
}

impl DriveTrain {
    pub fn new(state: Arc<GlobalDriveTrainState>, left: Box<MotorController>,
               right: Box<MotorController>, robot_status: Arc<GlobalLifeState>) -> Self {
        let enabled = state.get_enabled();
        Self {
            state,
            left,
            right,
            robot_status,
            enabled,
        }
    }

    /// Runs a cycle of the drive train, instructing all motors to do what they did last time.
    pub fn run_cycle(&mut self) {
        if self.enabled && self.robot_status.is_alive() {
            // TODO
        } else {
            self.brake();
        }
    }

    /// Drives the robot at the supplied speeds.
    pub fn drive(&mut self, left_speed: f32, right_speed: f32) {
        if self.enabled && self.robot_status.is_alive() {
            self.left.set_speed(left_speed);
            self.right.set_speed(right_speed);
        } else if self.enabled {
            self.brake();
        }
    }

    /// Causes the robot to brake.
    pub fn brake(&mut self) {
        self.right.stop();
        self.left.stop();
    }

    /// Enables the `DriveTrain`.
    pub fn enable(&mut self) {
        self.state.set_enabled(true);
        self.enabled = true;
    }

    /// Disables the `DriveTrain`.
    pub fn disable(&mut self) {
        self.state.set_enabled(false);
        self.enabled = false;
        self.brake();
    }
}