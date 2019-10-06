use crate::motor_controllers::{MotorController, GlobalMotorState};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;

use serialport::{available_ports, open, SerialPort};
use std::thread;
use std::io::Write;
use std::thread::{yield_now, JoinHandle};
use byteorder::BigEndian;
use byteorder::ByteOrder;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ArduinoMessage {
    data: [u8; 12]
}

impl ArduinoMessage {
    pub fn new(command: u8, payload_1: u16, payload_2: u16) -> Self {
        let checksum = ((command as u32 + 0x0D as u32 + payload_1 as u32 + payload_2 as u32)
            % 256) as u8;
        let mut p1_buff = [0, 0];
        BigEndian::write_u16(&mut p1_buff, payload_1);
        let [p1_a, p1_b] = p1_buff;
        let mut p2_buff = [0, 0];
        BigEndian::write_u16(&mut p2_buff, payload_2);
        let [p2_a, p2_b] = p2_buff;
        let data = [
            0x0A, 0x0A, 0x0A,
            0x0D,
            checksum,
            command,
            0x0D,
            p1_a, p1_b,
            0x0D,
            p2_a, p2_b,
        ];

        Self { data }
    }
}

pub struct ArduinoMotor {
    channel: Sender<ArduinoMessage>,
    id: u8,
    state: Arc<GlobalMotorState>,
}

impl ArduinoMotor {
    pub fn new(channel: Sender<ArduinoMessage>, id: u8, state: Arc<GlobalMotorState>) -> Self {
        Self {
            channel,
            id,
            state,
        }
    }
}

impl MotorController for ArduinoMotor {
    fn set_speed(&mut self, new_speed: f32) {
        debug_assert!(new_speed <= 1.0 && new_speed >= -1.0);
        self.state.set_speed(new_speed);

        let speed = (new_speed * 255.0).abs() as u8;
        let dir = if new_speed < 0.0 { 1 } else { 0 };

        let message = ArduinoMessage::new(self.id, speed as u16, dir);

        info!("Speed: {}", speed);

        if let Err(e) = self.channel.send(message) {
            error!("{}", e);
        };
    }

    fn stop(&mut self) {
        self.set_speed(0.0);
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

pub struct Arduino {
    channel: Receiver<ArduinoMessage>,
    port: Box<dyn SerialPort>,
}

impl Arduino {
    pub fn new(channel: Receiver<ArduinoMessage>) -> Self {
        info!("Serials: {:?}", &available_ports().expect("No serial port"));
        let mut serialport = open(&available_ports().expect("No serial port")[0].port_name)
            .expect("Failed to open serial port");

        if let Err(e) = serialport.set_baud_rate(115200) {
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
                panic!("")
            }
        };
        info!("Arduino: {:x?}", val);
        if let Err(e) = self.port.write(&val.data) {
            error!("{}", e)
        };
        if let Err(e) = self.port.flush() {
            error!("{}", e)
        };

        self.read_data();
    }

    fn read_data(&mut self) {
        let mut buffer = [0; 256];
        if let Ok(num_received) = self.port.read(&mut buffer) {
            let string = std::str::from_utf8(&buffer).unwrap();
            info!("{}", string);
        }
    }
}

impl Drop for Arduino {
    fn drop(&mut self) {
        error!("Dropping Arduino!");
    }
}