use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;

use rocket::http::Status;

use crate::robot::RobotBuilder;
use crate::robot_map::*;

use super::*;

const TIMEOUT: u64 = 100;

