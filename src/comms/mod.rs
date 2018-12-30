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
mod io;


pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

trait CommandReader<T: Command> {
    fn read_command(&mut self, ) -> T;
}

trait Command<Controller> {
    fn handle(&self, &mut controller: Controller);
}