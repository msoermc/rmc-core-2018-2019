use std::{
    collections::VecDeque,
    fs::{
        create_dir_all,
        File,
        OpenOptions,
    },
    io::{
        BufWriter,
        Result,
        Write,
    },
    path::Path,
    sync::mpsc::{
        channel,
        Receiver,
        Sender,
        TryRecvError,
    },
};

use chrono::prelude::{
    DateTime,
    Utc,
};

use crate::{
    framework::Subsystem,
    comms::SendableMessage,
};

///////////////////////////////////////////////////////////////////////////////////////////////////
// struct Logger
///////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Logger {
    counter: u64,
    writer: Option<BufWriter<File>>,
    backlog: VecDeque<LogData>,
    log_receiver: Receiver<LogData>,
    log_sender_template: Sender<LogData>,
    comms_channel: Option<Sender<Box<SendableMessage>>>,
}

impl Subsystem<LogData> for Logger {
    fn init(&mut self) {
        // Do nothing
    }

    fn run(&mut self) {
        // reestablish writer if not present
        if self.writer.is_none() {
            match get_file_to_use() {
                Ok(file) => {
                    let new_writer = BufWriter::new(file);
                    self.writer = Some(new_writer);
                },
                Err(_) => {
                    let severity = LogType::Error;
                    let description = "Could not get new log file!".to_string();
                    let timestamp = get_timestamp();

                    let error = LogData::new(severity, timestamp, description);
                    self.log_to_file(error.clone());
                    self.log_to_driver_station(error);
                },
            }
        }

        // log message from backlog
        if let Some(log_data) = self.backlog.pop_front() {
            self.log_to_file(log_data);
        }

        // log new message
        match self.log_receiver.try_recv() {
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
    }

    fn get_command_sender(&mut self) -> Sender<LogData> {
        self.log_sender_template.clone()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// impl Logger
///////////////////////////////////////////////////////////////////////////////////////////////////
impl Logger {
    pub fn new(comms_channel: Sender<Box<SendableMessage>>) -> Logger {
        let (log_sender_template, log_receiver) = channel();
        let file_result = get_file_to_use();

        let writer = match file_result {
            Ok(file) => Some(BufWriter::new(file)),
            Err(_) => None,
        };

        Logger {
            counter: 0,
            writer,
            backlog: VecDeque::new(),
            log_receiver,
            log_sender_template,
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

///////////////////////////////////////////////////////////////////////////////////////////////////
// fn get_file_to_use
///////////////////////////////////////////////////////////////////////////////////////////////////
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

///////////////////////////////////////////////////////////////////////////////////////////////////
// enum LogType
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogType {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// struct LogData
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct LogData {
    severity: LogType,
    timestamp: DateTime<Utc>,
    description: String,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// LogData
///////////////////////////////////////////////////////////////////////////////////////////////////
impl LogData {
    pub fn get_severity(&self) -> &LogType {
        &self.severity
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn to_string(&self) -> String {
        let severity = match self.severity {
            LogType::Debug => "Debug",
            LogType::Info => "Info",
            LogType::Warning => "Warning",
            LogType::Error => "Error",
            LogType::Fatal => "Fatal",
        };

        let timestamp = &self.timestamp.to_string();
        let description = self.get_description();

        format!("[{}]\n[TIMESTAMP]:\t{}\n[DESCRIPTION]:\t{}\n\n", severity, timestamp, description)
    }

    pub fn new(severity: LogType, timestamp: DateTime<Utc>, description: String) -> LogData {
        LogData {
            severity,
            timestamp,
            description,
        }
    }
}

impl SendableMessage for LogData {
    fn encode(&self) -> String {
        let timestamp = self.timestamp.to_string();
        let severity = match self.severity {
            LogType::Debug => "debug",
            LogType::Info => "info",
            LogType::Warning => "warning",
            LogType::Error => "error",
            LogType::Fatal => "fatal",
        };
        let description = &self.description;
        "log".to_string() + &severity.to_string() + &timestamp.to_string() + description
    }
}

pub fn get_timestamp() -> DateTime<Utc> {
    Utc::now()
}