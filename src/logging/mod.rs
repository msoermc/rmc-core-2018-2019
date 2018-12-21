use std::collections::VecDeque;
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
use crate::logging::log_data::get_timestamp;
use crate::logging::log_data::LogData;
use crate::logging::log_data::LogType;

pub mod log_data;

pub struct Logger {
    counter: u64,
    writer: Option<BufWriter<File>>,
    backlog: VecDeque<LogData>,
    logging_queue: Receiver<LogData>,
    comms_channel: Option<Sender<Box<SendableMessage>>>,
}

impl Runnable for Logger {
    fn init(&mut self) {
        // Do nothing
    }

    fn run(&mut self) -> bool {
        // reestablish writer if not present
        if self.writer.is_none() {
            match get_file_to_use() {
                Ok(file) => {
                    let new_writer = BufWriter::new(file);
                    self.writer = Some(new_writer);
                }
                Err(_) => {
                    let severity = LogType::Error;
                    let description = "Could not get new log file!".to_string();
                    let timestamp = get_timestamp();

                    let error = LogData::new(severity, timestamp, description);
                    self.log_to_file(error.clone());
                    self.log_to_driver_station(error);
                }
            }
        }

        // log message from backlog
        if let Some(log_data) = self.backlog.pop_front() {
            self.log_to_file(log_data);
        }

        // log new message
        match self.logging_queue.try_recv() {
            Ok(log) => {
                self.log_to_file(log.clone());
                self.log_to_driver_station(log);
            }
            Err(e) => {
                if let TryRecvError::Disconnected = e {
                    let severity = LogType::Fatal;
                    let timestamp = get_timestamp();
                    let description = "Lost logging channel!".to_string();
                    let log = LogData::new(severity, timestamp, description);

                    self.log_to_file(log.clone());
                    self.log_to_driver_station(log);
                }
            }
        }

        if self.counter % 10 == 0 {
            if let Some(writer) = &mut self.writer {
                if writer.flush().is_err() {
                    let severity = LogType::Error;
                    let timestamp = get_timestamp();
                    let description = "Lost logging channel!\n We lost logs!!!".to_string();

                    let log = LogData::new(severity, timestamp, description);

                    self.log_to_file(log.clone());
                    self.log_to_driver_station(log);
                }
            }
        }

        true
    }
}

impl Logger {
    pub fn new(comms_channel: Sender<Box<SendableMessage>>, logging_queue: Receiver<LogData>) -> Logger {
        let file_result = get_file_to_use();

        let writer = match file_result {
            Ok(file) => Some(BufWriter::new(file)),
            Err(_) => None,
        };

        Logger {
            counter: 0,
            writer,
            backlog: VecDeque::new(),
            logging_queue,
            comms_channel: Some(comms_channel),
        }
    }

    fn log_to_driver_station(&mut self, report: LogData) {
        if let Some(comms) = &mut self.comms_channel {
            if comms.send(Box::new(report)).is_err() {
                self.comms_channel = None;

                let severity = LogType::Error;
                let description = "Lost comms channel!".to_string();
                let timestamp = get_timestamp();
                let error = LogData::new(severity, timestamp, description);

                self.log_to_file(error);
            }
        }
    }

    fn log_to_file(&mut self, log_data: LogData) {
        match &mut self.writer {
            Some(writer) => {
                if writeln!(writer, "{}", log_data.to_string()).is_err() {
                    self.writer = None;

                    let severity = LogType::Error;
                    let description = "Lost writer!".to_string();
                    let timestamp = get_timestamp();
                    let error = LogData::new(severity, timestamp, description);

                    self.log_to_driver_station(error.clone());

                    self.backlog.push_back(log_data);
                    self.backlog.push_back(error);
                }
            }
            None => self.backlog.push_back(log_data)
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

