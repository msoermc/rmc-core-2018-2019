//! ![uml](ml.svg)
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;

use std::env::args;

use crate::run_modes::demo_mode::run_demo_mode;
use crate::run_modes::run_drive_train::run_drive_train;
use log::LevelFilter;

/// The framework module contains traits and interfaces key to the entire system.
/// It's purpose is not well defined, and we plan to phase this out at some point.
pub mod framework;

/// The devices module contains code for interfacing with various peripheral devices employed by the robot.
/// This category includes sensors and motor controllers.
pub mod devices;

/// The run_modes module contains various modes for running the robot.
/// Most of the modes are for test purposes only.
/// Modes are meant to be switched out by altering the code in `main`.
pub mod run_modes;

/// The comms module contains all code for controlling the communications infrastructure.
pub mod comms;

/// The control module contains all code for the controlling logic of the physical robot.
/// This includes managing subsystems like the drive train and MH.
pub mod control;

/// The robot map is a file filled with key constants such as pin numbers and network ports that
/// may change over time.
/// It is used to make reconfiguring pinouts a simpler process.
pub mod robot_map;

fn main() {
    setup_logger();
    run_demo_mode();
}

fn setup_logger() {
    env_logger::Builder::new().filter(None, LevelFilter::Off);
    info!("Launched logger");
}