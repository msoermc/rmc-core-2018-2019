use crate::motor_controllers::{MotorController, GlobalMotorState};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;

use serialport::{available_ports, open, SerialPort};
use std::{io, thread};
use std::io::Write;
use std::thread::{yield_now, JoinHandle};

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
        self.state.set_speed(new_speed);
        let dir = if new_speed < 0.0 {
            100
        } else {
            0
        };

        let speed = (new_speed.abs() * 10.0) as u8 * 10;

        info!("Speed: {}", speed);

        if let Err(e) = self.channel.send(dir + speed + self.id) {
            error!("{}", e);
        };
    }

    fn stop(&mut self) {
        if let Err(e) = self.channel.send(self.id) {
            error!("{}", e);
        };
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
        info!("Serials: {:?}", &available_ports().expect("No serial port"));
        let mut serialport = open(&available_ports().expect("No serial port")[0].port_name)
            .expect("Failed to open serial port");

        if let Err(e) = serialport.set_baud_rate(9600) {
            error!("{}", e);
        };

        Self {
            channel,
            port: serialport,
        }
    }

    pub fn launch(mut self) -> JoinHandle<()> {
        thread::Builder::new().name("Arduino Thread".to_owned()).spawn(move || {
            loop {
                self.run();
                yield_now();
            }
        }).unwrap()
    }

    fn run(&mut self) {
        let val = match self.channel.recv() {
            Ok(x) => x,
            Err(e) => {
                error!("{}", e);
                0
            }
        };
        info!("Arduino: {}", val);
        if let Err(e) = self.port.write(&[val]) {
            error!("{}", e)
        };
        if let Err(e) = self.port.flush() {
            error!("{}", e)
        };
    }
}

impl Drop for Arduino {
    fn drop(&mut self) {
        error!("Dropping Arduino!");
    }
}