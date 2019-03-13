use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;

use rocket::http::Status;
use rocket::local::Client;

use crate::robot_map::*;
use crate::status::robot_state::GlobalRobotState;

use super::*;

#[cfg(test)]
mod initialization;

#[cfg(test)]
mod switching;

#[cfg(test)]
mod benchmarking;

#[cfg(test)]
mod dumper;

#[cfg(test)]
mod drive_train;

#[cfg(test)]
mod intake;

#[cfg(test)]
mod killing;

fn setup() -> (Arc<GlobalRobotState>, Client) {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let client = robot.launch().engage_testing_server();

    (state, client)
}