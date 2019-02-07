use std::sync::Arc;

use crate::devices::motor_controllers::MotorController;
use crate::mechatronics::drive_train::state::GlobalDriveTrainState;
use crate::status::life::GlobalLifeState;

pub mod state;

/// Manages and controls the drive train.
pub struct DriveTrain {
    state: Arc<GlobalDriveTrainState>,
    left: Box<MotorController>,
    right: Box<MotorController>,
    robot_status: Arc<GlobalLifeState>,
}

impl DriveTrain {
    pub fn new(state: Arc<GlobalDriveTrainState>, left: Box<MotorController>,
               right: Box<MotorController>, robot_status: Arc<GlobalLifeState>) -> Self {
        Self {
            state,
            left,
            right,
            robot_status,
        }
    }

    /// Runs a cycle of the drive train, instructing all motors to do what they did last time.
    pub fn run_cycle(&mut self) {
        if self.state.get_enabled() && self.robot_status.is_alive() {} else {
            self.brake();
        }
    }

    /// Drives the robot at the supplied speeds.
    pub fn drive(&mut self, left_speed: f32, right_speed: f32) {
        if self.state.get_enabled() && self.robot_status.is_alive() {
            self.left.set_speed(left_speed);
            self.right.set_speed(right_speed);
        } else {
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
    }

    /// Disables the `DriveTrain`.
    pub fn disable(&mut self) {
        self.state.set_enabled(false);
        self.brake();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}