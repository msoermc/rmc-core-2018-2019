use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Write;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

pub mod external_comms;
pub mod internal_comms;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum CommunicatorError {
    InvalidAddressError,
    DisconnectedListenerError,
}

struct Communicator {
    listener: TcpListener,
    clients: Vec<TcpStream>,
}


impl Communicator {
    fn from(address: &str, port: u16) -> Result<Communicator, CommunicatorError> {
        let parsed_address: IpAddr = match address.parse() {
            Ok(addr) => addr,
            Err(_) => return Err(CommunicatorError::InvalidAddressError),
        };

        let address = SocketAddr::new(parsed_address, port);
        let listener = TcpListener::bind(&address).unwrap();

        listener.set_nonblocking(true).expect("Could not set listener to be nonblocking!");

        Ok(Communicator {
            listener,
            clients: Vec::new(),
        })
    }

    fn check_connections(&mut self) -> Result<(), CommunicatorError> {
        let connection_result = self.listener.accept();
        match connection_result {
            Ok(potential_connection) => {
                let (socket, _) = potential_connection;
                socket.set_nonblocking(true).expect("Could not set socket to be nonblocking!");
                self.clients.push(socket);
                return Ok(());
            }
            Err(error) => {
                if error.kind() != ErrorKind::WouldBlock {
                    return Err(CommunicatorError::DisconnectedListenerError);
                } else {
                    return Ok(());
                }
            }
        }
    }

    fn send(&mut self, message: String) -> Result<(), Vec<CommunicatorError>> {
        // TODO add error handling
        for client in &mut self.clients {
            write!(client, "{}", message).expect("Could not write line");
            client.flush().expect("Failed to flush");
        }

        return Ok(());
    }

    fn send_line(&mut self, message: String) -> Result<(), Vec<CommunicatorError>> {
        self.send(message + "\n")
    }

    fn receive_next_lines(&mut self) {
        let mut lines = Vec::new();

        for client in &self.clients {
            unimplemented!() // TODO
        }
    }
}