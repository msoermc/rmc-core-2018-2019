pub mod command_io_controller;

mod io;
mod reading;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}