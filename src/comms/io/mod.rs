use crate::logging::log_data::LogData;

pub mod tcp;

pub trait IoServerManager {
    fn create(address: &str, port: u16) -> Self;

    fn check_connections(&mut self) -> Result<(), LogData>;

    fn send(&mut self, message: &str) -> Vec<LogData>;

    fn send_line(&mut self, message: String) -> Vec<LogData>;

    fn receive_next_lines(&mut self) -> Vec<Result<String, LogData>>;
}
