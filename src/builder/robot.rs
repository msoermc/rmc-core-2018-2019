use std::thread;
use std::thread::JoinHandle;

use rocket::local::Client;
use rocket::Rocket;


pub struct Robot {
    controller: JoinHandle<()>,
    bfr: Rocket,
    _bench: Option<JoinHandle<()>>,
    _monitor: JoinHandle<()>,
}

impl Robot {
    pub fn new(controller: JoinHandle<()>, bfr: Rocket, bench: Option<JoinHandle<()>>, monitor: JoinHandle<()>) -> Self {
        Self {
            controller,
            bfr,
            _bench: bench,
            _monitor: monitor,
        }
    }

    pub fn engage_production_server(self) {
        let bfr = self.bfr;
        let _rocket_thread = thread::Builder::new().name("Rocket Thread".to_string()).spawn(move || bfr.launch()).unwrap();
        self.controller.join().expect("Controller thread panicked!");
    }

    pub fn engage_testing_server(self) -> Client {
        Client::new(self.bfr).expect("Failed to launch client!")
    }
}