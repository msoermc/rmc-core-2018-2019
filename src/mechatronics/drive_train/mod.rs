use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::MotorState;
use crate::status::life::GlobalLifeStatus;
use crate::mechatronics::drive_train::state::GlobalDriveTrainState;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub mod state;

/// Manages and controls the drive train.
pub struct DriveTrain {
    state: Arc<GlobalDriveTrainState>,
    left: MotorGroup,
    right: MotorGroup,
    robot_status: GlobalLifeStatus,
}

impl DriveTrain {
    pub fn new(left: MotorGroup, right: MotorGroup, robot_status: GlobalLifeStatus, state: Arc<GlobalDriveTrainState>) -> DriveTrain {
        DriveTrain {
            state,
            left,
            right,
            robot_status,
        }
    }

    /// Runs a cycle of the drive train, instructing all motors to do what they did last time.
    pub fn run_cycle(&mut self) {
        if self.state.get_enabled() && self.robot_status.is_alive() {
            self.maintain_last();
        } else {
            self.brake();
        }
    }

    /// Drives the robot at the supplied speeds.
    pub fn drive(&mut self, left_speed: f32, right_speed: f32) {
        if self.state.get_enabled() && self.robot_status.is_alive() {
            self.left.set_speed(left_speed);
            self.right.set_speed(right_speed);
        } else {
            self.left.stop();
            self.right.stop();
        }
    }

    /// Causes the robot to brake.
    pub fn brake(&mut self) {
        self.left.stop();
        self.right.stop();
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

    pub fn get_motor_states(&self) -> Vec<MotorState> {
        let mut states = self.left.get_states();
        states.append(&mut self.right.get_states());
        states
    }

    fn maintain_last(&mut self) {
        self.left.maintain_last();
        self.right.maintain_last();
    }
}