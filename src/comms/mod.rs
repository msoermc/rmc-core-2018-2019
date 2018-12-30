use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Write;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::logging::log_data::LogData;

pub mod driver_station;
pub mod internal_comms;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

/// An object used to take care of the TCP IO and abstract other objects from those details.
struct Communicator {
    listener: TcpListener,
    clients: Vec<TcpStream>,
}


impl Communicator {
    fn from(address: &str, port: u16) -> Communicator {
        let parsed_address: IpAddr = match address.parse() {
            Ok(addr) => addr,
            Err(_) => panic!("Invalid address provided!"),
        };

        let address = SocketAddr::new(parsed_address, port);
        let listener = TcpListener::bind(&address).unwrap();

        listener.set_nonblocking(true).expect("Could not set listener to be nonblocking!");

        Communicator {
            listener,
            clients: Vec::new(),
        }
    }

    fn check_connections(&mut self) -> Result<(), LogData> {
        let connection_result = self.listener.accept();
        match connection_result {
            Ok(potential_connection) => {
                let (socket, _) = potential_connection;
                socket.set_nonblocking(true).expect("Could not set socket to be nonblocking!");
                self.clients.push(socket);
                Ok(())
            }
            Err(error) => {
                if error.kind() != ErrorKind::WouldBlock {
                    Err(LogData::warning("Client failed to connect to driver station comms!"))
                } else {
                    Ok(())
                }
            }
        }
    }

    fn send(&mut self, message: &str) -> Vec<LogData> {
        let mut errors = Vec::new();
        for client_index in 0..self.clients.len() {
            if write!(self.clients[client_index], "{}", message).is_err() {
                let log = LogData::warning("Failed to write to client!");
                errors.push(log);
                self.clients.remove(client_index);
                continue;
            };
            if self.clients[client_index].flush().is_err() {
                let log = LogData::warning("Failed to flush data to client!");
                errors.push(log);
                self.clients.remove(client_index);
            };
        }

        errors
    }

    fn send_line(&mut self, message: String) -> Vec<LogData> {
        self.send(&(message + "\n"))
    }

    fn receive_next_lines(&mut self) -> Vec<Result<String, LogData>> {
        let mut lines = Vec::new();

        for client in &self.clients {
            let mut reader = BufReader::new(client);
            let mut buffer = String::new();

            if let Err(error) = reader.read_line(&mut buffer) {
                if error.kind() != ErrorKind::WouldBlock {
                    lines.push(Err(LogData::error("Failed to read from stream!")));
                }
            } else {
                lines.push(Ok(buffer));
            }
        }

        lines
    }
}

pub fn get_wrong_arg_count_log(message: &str, expected: u64, actual: u64) -> LogData {
    let description = format!(
        "Wrong number of elements in message '{}'. Expected {} args, instead got {}!",
        message, expected, actual);

    LogData::warning(description.as_str())
}
