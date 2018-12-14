use crate::framework::Subsystem;
use crate::framework::logging::LogData;
use std::sync::mpsc::Sender;

pub enum Message {
    Log(LogData),
}

pub struct Communicator {

}

impl Subsystem<Message> for Communicator {
    fn init(&mut self) {
        unimplemented!()
    }

    fn run(&mut self) {
        unimplemented!()
    }

    fn get_command_sender(&mut self) -> Sender<Message> {
        unimplemented!()
    }
}