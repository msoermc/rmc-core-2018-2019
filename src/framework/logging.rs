use std::fs::{
    File,
    OpenOptions,
};
use std::io::{
    BufWriter,
    Result,
    Write,
};
use std::path::Path;
use std::sync::mpsc::{
    channel,
    Receiver,
    Sender,
};
use std::thread::{
    JoinHandle,
    spawn,
    Thread,
};

use chrono::prelude;

struct Logger {
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
        let current_time = prelude::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // Use the current date and time to create a new log file
        let file_name = format!("~/RMC_Logs/RMC_Log_{}", current_time);
        
        // TODO make this more resilient before competition by retrying file creation
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
                    writeln!(writer, "{}", new_message.to_string());
                }
                
                if flush_counter % 10 == 0 {
                    writer.flush();
                }
                
                flush_counter += 1;
            }
        });
        
        logging_thread.join().unwrap()
    }
}

fn get_file_to_use(path: &Path) -> Result<File> {
    OpenOptions::new()
        .create_new(true)
        .write(true).open(path)
}

pub enum LogType {
    Debug(),
    Info(),
    Warning(),
    Error(),
    Fatal(),
}


pub struct LogData {
    severity: LogType,
    short_description: String,
    full_description: Option<String>,
}


impl LogData {
    pub fn get_severity(&self) -> &LogType {
        &self.severity
    }
    
    
    pub fn get_short_description(&self) -> &str {
        &self.short_description
    }
    
    
    pub fn get_full_description(&self) -> Option<&str> {
        match &self.full_description {
            Some(des) => Option::Some(des.as_str()),
            None => None
        }
    }
    
    // TODO implement
    pub fn to_string(&self) -> String {
        unimplemented!()
    }
}