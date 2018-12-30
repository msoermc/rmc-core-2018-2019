use crate::comms::SendableMessage;
use std::sync::mpsc::Sender;
use crate::logging::LogAccepter;
use crate::logging::log_data::LogData;

#[derive(Clone, Debug)]
pub struct DSMessageSender {
    channel: Sender<Box<SendableMessage>>
}

impl LogAccepter for DSMessageSender {
    fn accept_log(&mut self, log: LogData) {
        self.send(Box::new(log));
    }
}

impl DSMessageSender {
    pub fn new(channel: Sender<Box<SendableMessage>>) -> Self {
        DSMessageSender {
            channel
        }
    }

    pub fn send(&mut self, message: Box<SendableMessage>) {
        self.channel.send(message).expect("DSMessageSender hangup");
    }
}