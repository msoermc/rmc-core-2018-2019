use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use std::string::ToString;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;

use chrono::Utc;

use crate::comms::SendableMessage;
use crate::framework::Runnable;
use crate::logging::log_data::LogData;

pub mod log_data;

const LOG_FLUSH_PERIOD: u64 = 16;

pub struct Logger {
    counter: u64,
    writer: BufWriter<File>,
    logging_queue: Receiver<LogData>,
    comms_channel: Sender<Box<SendableMessage>>,
}

impl Runnable for Logger {
    fn init(&mut self) {
        // Do nothing
    }

    fn run(&mut self) -> bool {
        match self.logging_queue.try_recv() {
            Ok(log) => {
                self.comms_channel
                    .send(Box::new(log.clone()))
                    .expect("Comms channel disconnected!");
                writeln!(self.writer, "{}", log.to_string()).expect("Could not write log!");
            },
            Err(TryRecvError::Disconnected) => {
                panic!("Logging channel disconnected!");
            },
            Err(TryRecvError::Empty) => {
            }
        }

        if self.counter % LOG_FLUSH_PERIOD == 0 {
            self.writer.flush().expect("Could not flush logger!");
        }

        true
    }
}

impl Logger {
    pub fn new(comms_channel: Sender<Box<SendableMessage>>, logging_queue: Receiver<LogData>) -> Logger {
        let file = get_file_to_use().unwrap();

        let writer =  BufWriter::new(file);

        Logger {
            counter: 0,
            writer,
            logging_queue,
            comms_channel,
        }
    }
}

fn get_file_to_use() -> Result<File> {
    let current_time = Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string().trim().to_string();
    let file_name = format!("./RMC_Logs/{}.log", current_time);
    let path = Path::new(&file_name);

    create_dir_all(path.parent().unwrap())?;
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
}

