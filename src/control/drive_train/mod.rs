use std::sync::Arc;
use std::sync::RwLock;

use crate::control::RobotLifeStatus;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::MotorState;

#[cfg(test)]
mod tests;

/// Manages and controls the drive train.
pub struct DriveTrain {
    is_enabled: bool,
    left: MotorGroup,
    right: MotorGroup,
    robot_status: Arc<RwLock<RobotLifeStatus>>,
}

impl DriveTrain {
    pub fn new(left: MotorGroup, right: MotorGroup, robot_status: Arc<RwLock<RobotLifeStatus>>) -> DriveTrain {
        DriveTrain {
            is_enabled: true,
            left,
            right,
            robot_status,
        }
    }

    /// Runs a cycle of the drive train, instructing all motors to do what they did last time.
    pub fn run_cycle(&mut self) -> Result<(), Vec<MotorState>> {
        let mut errors = Vec::new();

        if self.is_enabled && *self.robot_status.read().expect("Drive train failed to read life") == RobotLifeStatus::Alive {
            if let Err(e) = &mut self.maintain_last() {
                errors.append(e);
            }
        } else if let Err(e) = &mut self.brake() {
            errors.append(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Drives the robot at the supplied speeds.
    pub fn drive(&mut self, left_speed: f32, right_speed: f32) -> Result<(), Vec<MotorState>> {
        let mut errors = Vec::new();
        if *self.robot_status.read().unwrap() == RobotLifeStatus::Alive && self.is_enabled {
            if let Err(e) = &mut self.left.set_speed(left_speed) {
                errors.append(e);
            }

            if let Err(e) = &mut self.right.set_speed(right_speed) {
                errors.append(e);
            }
        } else {
            if let Err(e) = &mut self.left.stop() {
                errors.append(e);
            }

            if let Err(e) = &mut self.right.stop() {
                errors.append(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Causes the robot to brake.
    pub fn brake(&mut self) -> Result<(), Vec<MotorState>> {
        let mut errors = Vec::new();
        if let Err(e) = &mut self.left.stop() {
            errors.append(e);
        }

        if let Err(e) = &mut self.right.stop() {
            errors.append(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Enables the `DriveTrain`.
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    /// Disables the `DriveTrain`.
    pub fn disable(&mut self) -> Result<(), Vec<MotorState>> {
        self.is_enabled = false;
        self.brake()
    }

    fn maintain_last(&mut self) -> Result<(), Vec<MotorState>> {
        let mut errors = Vec::new();
        if let Err(e) = &mut self.left.maintain_last() {
            errors.append(e);
        }
        if let Err(e) = &mut self.right.maintain_last() {
            errors.append(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}