//! ![uml](ml.svg)
#![feature(proc_macro_hygiene, decl_macro)]

extern crate atomic;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use(o)]
extern crate slog;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

/// Contains all code responsible for monitoring system power.
pub mod power;

/// The framework module contains traits and interfaces key to the entire system.
/// It's purpose is not well defined, and we plan to phase this out at some point.
pub mod framework;

/// Contains traits, functions, and structures used for pinouts.
pub mod pinouts;

/// Contains all structs and traits for operating different motor controllers, as well as the motor
/// state structs.
pub mod motor_controllers;

/// Contains all code for sensors more complicated than a limit switch.
pub mod sensors;

/// The comms module contains the code for running the HTTP server
pub mod comms;

/// The mechatronics module contains all code for the controlling logic of the physical robot.
/// This includes managing subsystems like the drive train, intake and dumper.
pub mod mechatronics;

/// The robot map is a file filled with key constants such as pin numbers and network ports that
/// may change over time.
/// It is used to make reconfiguring pinouts a simpler process.
pub mod robot_map;

/// Used for building the robot and assembling all components together.
pub mod robot;

/// Contains code used to start the global logger.
pub mod logging;

/// Contains structures used to represent portions of the state of the robot, including the code for
/// the life status and global state.
pub mod status;

/// Contains integration tests which test the full stack of the software.
#[cfg(test)]
mod integration_tests;

fn main() {
    let _logging_guard = logging::launch_logger();
    let mut robot_builder = robot::RobotBuilder::new();
    //robot_builder.add_real_drive();
    robot_builder.build().launch();
}