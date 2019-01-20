use std::sync::Arc;
use std::sync::RwLock;

use crate::control::RobotLifeStatus;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::MotorFailure;

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
    pub fn run_cycle(&mut self) -> Result<(), Vec<MotorFailure>> {
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
    pub fn drive(&mut self, left_speed: f32, right_speed: f32) -> Result<(), Vec<MotorFailure>> {
        let mut errors = Vec::new();
        if *self.robot_status.read().unwrap() == RobotLifeStatus::Alive {
            if self.is_enabled {
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
    pub fn brake(&mut self) -> Result<(), Vec<MotorFailure>> {
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
    pub fn disable(&mut self) -> Result<(), Vec<MotorFailure>> {
        self.is_enabled = false;
        self.brake()
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

#[cfg(test)]
mod tests {
    use crate::devices::motor_controllers::test_motor::TestMotor;

    use super::*;

    struct TestMotorGroup {
        pub inverted: Arc<RwLock<bool>>,
        pub speed: Arc<RwLock<f32>>,
        pub motor_group: MotorGroup,
    }

    fn test_cycle_no_fail_no_inversion() {
        let (left, right) = create_groups();
        let status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

        let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status.clone());

        // Make sure we are setup correctly
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test both forwards
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Test cycle
        drive_train.run_cycle().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Test brake
        drive_train.brake().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test cycle
        drive_train.run_cycle().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test both forwards
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Kill
        *status.write().unwrap() = RobotLifeStatus::Dead;

        // Test cycle
        drive_train.run_cycle().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Revive
        *status.write().unwrap() = RobotLifeStatus::Alive;

        // Test cycle
        drive_train.drive(1.0, 1.0).unwrap();
        drive_train.run_cycle().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Disable
        drive_train.disable().unwrap();

        // Test cycle
        drive_train.run_cycle().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Enable
        drive_train.enable();

        // Test cycle
        drive_train.drive(1.0, 1.0).unwrap();
        drive_train.run_cycle().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());
    }


    #[test]
    fn test_drive_no_fail_no_inversion() {
        let (left, right) = create_groups();
        let status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

        let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status);

        // Make sure we are setup correctly
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test both forwards
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Test both backwards
        drive_train.drive(-1.0, -1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(-1.0, *left.speed.read().unwrap());
        assert_eq!(-1.0, *right.speed.read().unwrap());

        // Test right forwards and left backwards
        drive_train.drive(-1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(-1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Test right backwards and left forwards
        drive_train.drive(1.0, -1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(-1.0, *right.speed.read().unwrap());
    }

    #[test]
    fn test_brake_no_fail_no_inversion() {
        let (left, right) = create_groups();
        let status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

        let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status);

        // Make sure we are setup correctly
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test both forwards
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Test brake
        drive_train.brake().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());
    }

    #[test]
    fn test_enabling_no_fail_no_inversion() {
        let (left, right) = create_groups();
        let status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

        let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status);

        // Make sure we are setup correctly
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test both forwards
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Test disable
        drive_train.disable().expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Make sure we can't still drive
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test enable
        drive_train.enable();
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Make sure we can drive
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());
    }

    #[test]
    fn test_killing_no_fail_no_inversion() {
        let (left, right) = create_groups();
        let status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

        let mut drive_train = DriveTrain::new(left.motor_group, right.motor_group, status.clone());

        // Make sure we are setup correctly
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test both forwards
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());

        // Test kill
        *status.write().unwrap() = RobotLifeStatus::Dead;
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Test revive
        *status.write().unwrap() = RobotLifeStatus::Alive;
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(0.0, *left.speed.read().unwrap());
        assert_eq!(0.0, *right.speed.read().unwrap());

        // Make sure we can drive
        drive_train.drive(1.0, 1.0).expect("Drive command had not reason to fail!");
        assert_eq!(false, *left.inverted.read().unwrap());
        assert_eq!(false, *right.inverted.read().unwrap());

        assert_eq!(1.0, *left.speed.read().unwrap());
        assert_eq!(1.0, *right.speed.read().unwrap());
    }

    fn create_groups() -> (TestMotorGroup, TestMotorGroup) {
        let inverted_0 = Arc::new(RwLock::new(false));
        let inverted_1 = Arc::new(RwLock::new(false));

        let speed_0 = Arc::new(RwLock::new(0.0));
        let speed_1 = Arc::new(RwLock::new(0.0));

        let test_motor_0 = TestMotor::new(inverted_0.clone(), speed_0.clone());
        let test_motor_1 = TestMotor::new(inverted_1.clone(), speed_1.clone());

        let test_group_0 = MotorGroup::new(vec![Box::new(test_motor_0)]);
        let test_group_1 = MotorGroup::new(vec![Box::new(test_motor_1)]);

        let test_unit_0 = TestMotorGroup { inverted: inverted_0, speed: speed_0, motor_group: test_group_0 };
        let test_unit_1 = TestMotorGroup { inverted: inverted_1, speed: speed_1, motor_group: test_group_1 };

        (test_unit_0, test_unit_1)
    }
}