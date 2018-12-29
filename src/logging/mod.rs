use crate::logging::log_data::LogData;

pub mod log_data;
pub mod log_sender;
pub mod log_manager;

pub trait LogAccepter: Send {
    fn accept_log(&mut self, log: LogData);
}