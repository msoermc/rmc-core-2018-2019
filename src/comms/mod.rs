pub mod command_io_controller;

pub mod driver_station;
mod io;
mod reading;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}