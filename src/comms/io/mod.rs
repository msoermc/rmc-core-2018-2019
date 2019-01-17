use crate::logging::log_data::LogData;

/// Contains an implementation of the `IoServerManager` using a TCP server.
pub mod tcp;

/// Abstracts away the implementation of an IO interface from the rest of the comms system.
pub trait IoServerManager {
    /// Creates a new IoServerManager using the specified port and address.
    fn create(address: &str, port: u16) -> Self;

    /// Checks our connections, returning LogData if something happens.
    fn check_connections(&mut self) -> Result<(), LogData>;

    /// Sends a message to all endpoints, returning a vector of logs.
    fn send(&mut self, message: &str) -> Vec<LogData>;

    /// Sends a message with a newline terminator, returning a vector of logs.
    fn send_line(&mut self, message: String) -> Vec<LogData>;

    /// Receives the next message from each remote host and returns them.
    fn receive_next_lines(&mut self) -> Vec<Result<String, LogData>>;
}
