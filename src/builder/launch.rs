use std::thread;

use rocket::local::Client;
use rocket::Rocket;

use crate::benchmarking::ControllerBench;
use crate::framework::Runnable;
use crate::mechatronics::controller::RobotController;

pub struct RobotLauncher {
    controller: RobotController,
    bfr: Rocket,
    bench: Option<ControllerBench>,
}

impl RobotLauncher {
    pub fn new(controller: RobotController, bfr: Rocket, bench: Option<ControllerBench>) -> Self {
        Self {
            controller,
            bfr,
            bench,
        }
    }

    /// Launches the robot, taking over the current thread.
    /// This method consumes the robot.
    pub fn launch(self) {
        let bfr = self.bfr;
        let mut controller = self.controller;
        let controller_thread = thread::Builder::new().name("Controller Thread".to_string()).spawn(move || controller.start()).unwrap();
        let _rocket_thread = thread::Builder::new().name("Rocket Thread".to_string()).spawn(move || bfr.launch()).unwrap();
        self.bench.map(|bench| thread::Builder::new().name("Bench Thread".to_string()).spawn(move || {
            bench.launch();
        }).unwrap());

        controller_thread.join().expect("Controller thread panicked!");
    }

    /// Launches the robot in test mode in a separate thread.
    /// This method consumes the robot and returns a `Client` object which can be used for sending requests
    /// to the robot.
    pub fn launch_tester(self) -> Client {
        let bfr = self.bfr;
        let mut controller = self.controller;
        thread::Builder::new().name("Controller Thread".to_string()).spawn(move || controller.start()).unwrap();
        self.bench.map(|bench| thread::Builder::new().name("Bench Thread".to_string()).spawn(move || {
            bench.launch();
        }).unwrap());
        Client::new(bfr).expect("Failed to launch client!")
    }
}