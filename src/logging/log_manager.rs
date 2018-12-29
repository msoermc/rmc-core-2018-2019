use std::io::BufWriter;
use std::fs::File;
use std::sync::mpsc::Receiver;
use crate::logging::log_data::LogData;
use crate::comms::SendableMessage;
use std::sync::mpsc::Sender;
use crate::framework::Runnable;
use std::sync::mpsc::TryRecvError;
use std::io::Write;
use std::fs::OpenOptions;
use std::fs::create_dir_all;
use chrono::Utc;
use std::path::Path;
use std::io::Result;
use crate::logging::LogAccepter;
use crate::logging::log_sender::LogSender;
use std::sync::mpsc::channel;

pub struct LogManager {
    flush_period: u64,
    counter: u64,
    writer: BufWriter<File>,
    logging_queue: Receiver<LogData>,
    log_sender: Sender<LogData>,
    downstream: Vec<Box<LogAccepter>>,
}

impl Runnable for LogManager {
    fn init(&mut self) {
        // Do nothing
    }

    fn run(&mut self) -> bool {
        match self.logging_queue.try_recv() {
            Ok(log) => {
                for accepter in &mut self.downstream {
                    accepter.accept_log(log.clone());
                }
                writeln!(self.writer, "{}", log.to_string()).expect("Could not write log!");
            }
            Err(TryRecvError::Disconnected) => {
                panic!("Logging channel disconnected!");
            }
            Err(TryRecvError::Empty) => {}
        }

        if self.counter % self.flush_period == 0 {
            self.writer.flush().expect("Could not flush logger!");
        }

        true
    }
}

impl LogManager {
    pub fn new(filepath: &str, flush_period: u64) -> LogManager {
        let (log_sender, logging_queue) = channel();
        let file = get_file_to_use(filepath).unwrap();
        let writer = BufWriter::new(file);

        LogManager {
            flush_period,
            counter: 0,
            writer,
            logging_queue,
            log_sender,
            downstream: Vec::new(),
        }
    }

    pub fn attach_accepter(&mut self, accepter: Box<LogAccepter>) {
        self.downstream.push(accepter);
    }

    pub fn get_sender(&mut self) -> LogSender {
        LogSender::new(self.log_sender.clone())
    }
}

fn get_file_to_use(filepath: &str) -> Result<File> {
    let current_time = Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string().trim().to_string();
    let file_name = format!("{}/{}.log", filepath, current_time);
    let path = Path::new(&file_name);

    create_dir_all(path.parent().unwrap())?;
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
}

