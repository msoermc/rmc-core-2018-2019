use std::thread;

use rocket::Rocket;

use crate::benchmarking::ControllerBench;
use crate::framework::{Runnable, CompositeRunnable};
use crate::mechatronics::controller::RobotController;
use crate::builder::robot::Robot;
use crate::arduino::Arduino;

pub struct RobotLauncher {
    controller: RobotController,
    bfr: Rocket,
    bench: Option<ControllerBench>,
    monitor: CompositeRunnable,
    arduino: Option<Arduino>,
}

impl RobotLauncher {
    pub fn new(controller: RobotController, bfr: Rocket, bench: Option<ControllerBench>, monitor: CompositeRunnable, arduino: Option<Arduino>) -> Self {
        Self {
            controller,
            bfr,
            bench,
            monitor,
            arduino,
        }
    }

    /// Launches the robot, taking over the current thread.
    /// This method consumes the robot.
    pub fn launch(self) -> Robot {
        let mut controller = self.controller;
        let mut monitor = self.monitor;
        let controller_thread = thread::Builder::new().name("Controller Thread".to_string()).spawn(move || controller.start()).unwrap();
        let bench_thread = self.bench.map(|bench| thread::Builder::new().name("Bench Thread".to_string()).spawn(move || {
            bench.launch();
        }).unwrap());

        let arduino_thread = self.arduino.map(|ard| ard.launch());

        let monitor_thread = thread::Builder::new().name("Monitor Thread".to_string()).spawn(move || monitor.start()).unwrap();

        Robot::new(controller_thread, self.bfr, bench_thread, monitor_thread, arduino_thread)
    }
}