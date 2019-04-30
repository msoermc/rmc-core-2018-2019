//! ![uml](ml.svg)
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(clippy::new_without_default)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use(o)]
extern crate slog;

pub mod arduino;

use crate::builder::config::RobotAssemblyBuilder;

/// Contains code for benchmarking the performance of the system.
pub mod benchmarking;

/// The framework module contains traits and interfaces key to the entire system.
/// It's purpose is not well defined, and we plan to phase this out at some point.
pub mod framework;

/// Contains traits, functions, and structures used for pinouts.
pub mod pinouts;

/// Contains all structs and traits for operating different motor controllers, as well as the motor
/// state structs.
pub mod motor_controllers;

/// The comms module contains the code for running the HTTP server
pub mod comms;

/// The mechatronics module contains all code for the controlling logic of the physical robot.
/// This includes managing subsystems like the drive train, intake and dumper.
pub mod mechatronics;

/// The robot map is a file filled with key constants such as pin numbers and network ports that
/// may change over time.
/// It is used to make reconfiguring pinouts a simpler process.
pub mod robot_map;

/// Contains code used to start the global logger.
pub mod logging;

/// Contains structures used to represent portions of the state of the robot, including the code for
/// the life status and global state.
pub mod status;

pub mod builder;

pub mod sensors;

/// Contains integration tests which test the full stack of the software.
#[cfg(test)]
mod integration_tests;

fn main() {
    let _logging_guard = logging::launch_logger();
    let mut builder = RobotAssemblyBuilder::new();

    builder.with_arduino();
    builder.with_production();

    builder.generate()
        .assemble()
        .launch()
        .engage_production_server();
}