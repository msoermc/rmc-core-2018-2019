use crate::builder::factories::SubsystemFactory;
use crate::arduino::{Arduino, ArduinoMessage};
use std::sync::mpsc::Receiver;

pub struct ArduinoFactory {
    channel: Receiver<ArduinoMessage>,
}

impl ToString for ArduinoFactory {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

impl SubsystemFactory<Arduino> for ArduinoFactory {
    fn produce(self: Box<Self>) -> Arduino {
        Arduino::new(self.channel)
    }
}