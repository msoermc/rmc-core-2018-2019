use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::MotorFailure;
use crate::control::RobotLifeStatus;

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

    pub fn run_cycle(&mut self) -> Result<(), Vec<MotorFailure>> {
        let mut errors = Vec::new();

        if self.is_enabled && *self.robot_status.read().expect("Drive train failed to read life") == RobotLifeStatus::Alive {
            if let Err(e) = &mut self.maintain_last() {
                errors.append(e);
            }
        } else if let Err(e) = &mut self.stop() {
            errors.append(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn drive(&mut self, left_speed: f32, right_speed: f32) -> Result<(), Vec<MotorFailure>> {
        let mut errors = Vec::new();
        if let Err(e) = &mut self.left.set_speed(left_speed) {
            errors.append(e);
        }

        if let Err(e) = &mut self.right.set_speed(right_speed) {
            errors.append(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn stop(&mut self) -> Result<(), Vec<MotorFailure>> {
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

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) -> Result<(), Vec<MotorFailure>> {
        self.is_enabled = false;
        self.stop()
    }

    fn maintain_last(&mut self) -> Result<(), Vec<MotorFailure>> {
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
