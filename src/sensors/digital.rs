use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use atomic::Ordering;

use crate::framework::Runnable;
use crate::pinouts::digital::input::DigitalInput;

pub struct DigitalInputMonitor {
    input: Box<DigitalInput>,
    update_field: Arc<AtomicBool>,
}

impl Runnable for DigitalInputMonitor {
    fn init(&mut self) {
        self.update_field.store(self.input.get_value(), Ordering::SeqCst)
    }

    fn run(&mut self) {
        self.update_field.store(self.input.get_value(), Ordering::SeqCst)
    }
}

impl DigitalInputMonitor {
    pub fn new(input: Box<DigitalInput>, update_field: Arc<AtomicBool>) -> Self {
        Self {
            input,
            update_field,
        }
    }
}