///////////////////////////////////////////////////////////////////////////////////////////////////
// std imports
///////////////////////////////////////////////////////////////////////////////////////////////////
use std::sync::mpsc::{
    channel,
    Sender,
    Receiver,
};

use std::fs::{
    File,
    OpenOptions,
};

use std::thread::{
    Thread,
    spawn,
    JoinHandle
};
use std::path::Path;
use std::io::Result;

///////////////////////////////////////////////////////////////////////////////////////////////////
// External crate imports
///////////////////////////////////////////////////////////////////////////////////////////////////
use chrono::prelude;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Internal imports from this crate
///////////////////////////////////////////////////////////////////////////////////////////////////
use super::{
    LogData,
    LogType,
};

pub struct Logger {
    file: File,
    log_receiver: Receiver<LogData>,
    log_sender_template: Sender<LogData>,
}

impl Logger {
    fn new() -> Logger {
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
    
    fn get_sender(&self) -> Sender<LogData> {
        self.log_sender_template.clone()
    }
    
    fn start(self) -> JoinHandle<Thread> {
        let logging_thread = spawn(|| {
            let receiver = self.log_receiver;
            loop {
                if receiver.can_recv() {
                    let new_message = receiver.try_recv().unwrap();
                    let message_string = new_message.
                }
            }
        });
        
        logging_thread.join()
    }
}

fn get_file_to_use(path: &Path) -> Result<File> {
    OpenOptions::new()
        .create_new(true)
        .write(true).open(path)
}