//! ![uml](ml.svg)
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use(o)]
extern crate slog;

use libbeaglebone::prelude::*;
use std::time::Duration;
use std::thread;
use crate::pinouts::enable_pins;

/// Contains all code responsible for monitoring system power.
pub mod power;

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

//fn main() {
//    let _logging_guard = logging::launch_logger();
//    let mut robot_builder = robot::RobotBuilder::new();
////    robot_builder.with_bench();
//    robot_builder.with_real();
//    robot_builder.build().launch();
//}

fn main() {
    enable_pins().unwrap();
    // Create a GPIO object at pin #69 that'll represent the LED, export it, and
    // set it as an output
    // Adjust the pin number to whatever pin your LED is connected to
    let mut led = GPIO::new(GPIO_P8_9);
    led.set_export(DeviceState::Exported).unwrap();
    led.set_direction(PinDirection::Out).unwrap();

    for _ in 1..11 {
        // Toggle the LED on and off every 250ms 10 times
        led.write(PinState::High).unwrap();
        thread::sleep(Duration::from_millis(250));
        led.write(PinState::Low).unwrap();
        thread::sleep(Duration::from_millis(250));
    }

    // Unexport the LED once we're done with it.
    led.set_export(DeviceState::Unexported).unwrap();
}
