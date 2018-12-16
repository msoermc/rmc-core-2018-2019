use crate::framework::Subsystem;
use crate::framework::logging::LogData;
use std::sync::mpsc::Sender;
use std::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:343";

pub enum Message {
    Log(LogData),
}

pub enum ProtocolSubsystem {
    DriveTrain,
}

pub struct Communicator {
    sending_queue: Sender<Message>,
}

impl Communicator {
    pub fn new(logger_channel: Sender<LogData>, sending_channel: Sender<Message>) -> Communicator {
        unimplemented!()
    }

    pub fn start(self) {
        unimplemented!()
    }

    fn handle_new_network_message() {

    }

    fn handle_kill(&mut self) {
        unimplemented!()
    }

    fn handle_revive(&mut self) {
        unimplemented!()
    }

    fn handle_enable(&mut self, subsystem: ProtocolSubsystem) {
        unimplemented!()
    }

    fn handle_disable(&mut self, subsystem: ProtocolSubsystem) {
        unimplemented!()
    }

    fn handle_brake(&mut self) {
        unimplemented!()
    }

    fn handle_drive(&mut self) {
        unimplemented!()
    }

    fn handle_log(&mut self) {
        unimplemented!()
    }
}