use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Write;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::framework::logging::LogData;

pub mod driver_station;
pub mod internal_comms;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

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

    fn send(&mut self, message: &str) -> Result<(), Vec<LogData>> {
        // TODO add error handling
        for client in &mut self.clients {
            write!(client, "{}", message).expect("Could not write line");
            client.flush().expect("Failed to flush");
        }

        Ok(())
    }

    fn send_line(&mut self, message: String) -> Result<(), Vec<LogData>> {
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
