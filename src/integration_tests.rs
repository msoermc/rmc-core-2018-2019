use std::sync::Arc;
use std::sync::RwLock;

use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::test_motor::TestMotor;
use crate::robot_map::*;

use rocket::http::Status;

use super::*;
use crate::robot::RobotBuilder;
use std::time::Duration;
use std::thread::sleep;
use crate::devices::motor_controllers::GlobalMotorState;

const TIMEOUT: u64 = 100;

