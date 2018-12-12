use std::{
    fs::{
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
    },
    thread::{
        JoinHandle,
        spawn,
        Thread,
    },
};

use chrono::prelude::{
    DateTime,
    Utc,
};

pub struct Logger {
    file: File,
    log_receiver: Receiver<LogData>,
    log_sender_template: Sender<LogData>,
}

impl Logger {
    pub fn new() -> Logger {
        let channel_pair = channel();
        let log_sender_template = channel_pair.0;
        let log_receiver = channel_pair.1;


        // Get the current utc time in 'y-m-d h:m:s' form
        let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Use the current date and time to create a new log file
        let file_name = format!("~/RMC_Logs/RMC_Log_{}", current_time);

        let file = get_file_to_use(Path::new(&file_name)).unwrap();

        Logger {
            file,
            log_receiver,
            log_sender_template,
        }
    }

    pub fn get_sender(&self) -> Sender<LogData> {
        self.log_sender_template.clone()
    }

    pub fn start(self) -> JoinHandle<Thread> {
        let logging_thread = spawn(|| {
            let receiver = self.log_receiver;
            let mut writer = BufWriter::new(self.file);
            let mut flush_counter: u64 = 0;

            loop {
                if let Ok(new_message) = receiver.recv() {
                    writeln!(writer, "{}", new_message.to_string()).unwrap();
                }

                if flush_counter % 10 == 0 {
                    writer.flush().unwrap();
                }

                flush_counter += 1;
            }
        });

        logging_thread.join().unwrap()
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

fn get_file_to_use(path: &Path) -> Result<File> {
    OpenOptions::new()
        .create_new(true)
        .write(true).open(path)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogType {
    Debug(),
    Info(),
    Warning(),
    Error(),
    Fatal(),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogData {
    severity: LogType,
    timestamp: DateTime<Utc>,
    description: String,
}


impl LogData {
    pub fn get_severity(&self) -> &LogType {
        &self.severity
    }


    pub fn get_description(&self) -> &str {
        &self.description
    }


    pub fn to_string(&self) -> String {
        let severity = match self.severity {
            LogType::Debug() => "Debug",
            LogType::Info() => "Info",
            LogType::Warning() => "Warning",
            LogType::Error() => "Error",
            LogType::Fatal() => "Fatal",
        };

        let timestamp = &self.timestamp.to_string();
        let description = self.get_description();

        format!("[{}]:\t{}\n{}", severity, timestamp, description)
    }
}