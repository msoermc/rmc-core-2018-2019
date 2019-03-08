use std::thread;

use rocket::local::Client;
use rocket::Rocket;

use crate::benchmarking::ControllerBench;
use crate::framework::{Runnable, CompositeRunnable};
use crate::mechatronics::controller::RobotController;
use crate::builder::robot::Robot;

pub struct RobotLauncher {
    controller: RobotController,
    bfr: Rocket,
    bench: Option<ControllerBench>,
    monitor: CompositeRunnable
}

impl RobotLauncher {
    pub fn new(controller: RobotController, bfr: Rocket, bench: Option<ControllerBench>, monitor: CompositeRunnable) -> Self {
        Self {
            controller,
            bfr,
            bench,
            monitor,
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

        let monitor_thread = thread::Builder::new().name("Monitor Thread".to_string()).spawn(move || monitor.start()).unwrap();

        Robot::new(controller_thread, self.bfr, bench_thread, monitor_thread)
    }
}