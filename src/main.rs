//! ![uml](ml.svg)
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;
#[macro_use(slog_o, slog_kv)]
extern crate slog;
extern crate slog_stdlog;
extern crate slog_scope;
extern crate slog_term;
extern crate slog_async;


use crate::run_modes::demo_mode::run_demo_mode;
use crate::run_modes::run_drive_train::run_drive_train;
use slog::Drain;

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
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, slog_o!("version" => env!("CARGO_PKG_VERSION")));

    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init().unwrap();
    info!("Launched logger");
}