use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::builder::factories::SubsystemFactory;
use crate::framework::Runnable;
use crate::pinouts::digital::input::DigitalInput;
use crate::sensors::digital::DigitalInputMonitor;

pub struct DigitalMonitorFactory {
    update_field: Arc<AtomicBool>,
    input: Box<DigitalInput>,
}

impl DigitalMonitorFactory {
    pub fn new(state: Arc<AtomicBool>, input: Box<DigitalInput>) -> Self {
        Self {
            update_field: state,
            input,
        }
    }
}


impl ToString for DigitalMonitorFactory {
    fn to_string(&self) -> String {
        "production digital monitor".to_owned()
    }
}

impl SubsystemFactory<Box<Runnable>> for DigitalMonitorFactory {
    fn produce(self: Box<Self>) -> Box<Runnable> {
        Box::new(DigitalInputMonitor::new(self.input, self.update_field, false))
    }
}