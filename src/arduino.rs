use crate::motor_controllers::{MotorController, GlobalMotorState};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use crate::framework::Runnable;

use serialport::{available_ports, open, SerialPort};
use std::{io, thread};
use std::io::Write;
use std::thread::yield_now;

pub struct ArduinoMotor {
    channel: Sender<u8>,
    id: u8,
    state: Arc<GlobalMotorState>,
}

impl ArduinoMotor {
    pub fn new(channel: Sender<u8>, id: u8, state: Arc<GlobalMotorState>) -> Self {
        Self {
            channel,
            id,
            state,
        }
    }
}

impl MotorController for ArduinoMotor {
    fn set_speed(&mut self, new_speed: f32) {
        let dir = if new_speed < 0.0 {
            100
        } else {
            0
        };

        let speed = (new_speed.abs() * 10.0) as u8;

        self.channel.send(dir + speed + self.id).unwrap();
    }

    fn stop(&mut self) {
        self.channel.send(self.id);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

pub struct Arduino {
    channel: Receiver<u8>,
    port: Box<SerialPort>,
}

impl Arduino {
    pub fn new(channel: Receiver<u8>) -> Self {
        let mut serialport = open(&available_ports().expect("No serial port")[0].port_name)
            .expect("Failed to open serial port");

        Self {
            channel,
            port: serialport,
        }
    }

    pub fn launch(mut self) {
        let arduino_thread =
            thread::Builder::new().name("Arduino Thread".to_owned()).spawn(move || {
                loop {
                    self.run();
                    yield_now();
                }
            });
    }

    fn run(&mut self) {
        let val = self.channel.recv().unwrap();
        if let Err(e) = self.port.write(&[val]) {
            error!("{}", e)
        };
        if let Err(e) = self.port.flush() {
            error!("{}", e)
        };
    }
}