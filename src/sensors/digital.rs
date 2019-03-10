use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use atomic::Ordering;

use crate::framework::Runnable;
use crate::pinouts::digital::input::DigitalInput;

pub struct DigitalInputMonitor {
    input: Box<DigitalInput>,
    update_field: Arc<AtomicBool>,
    default: bool,
}

impl Runnable for DigitalInputMonitor {
    fn init(&mut self) {
        if let Some(val) = self.input.get_value() {
            self.update_field.store(val, Ordering::SeqCst)
        } else {
            self.update_field.store(self.default, Ordering::SeqCst)
        }
    }

    fn run(&mut self) {
        if let Some(val) = self.input.get_value() {
            self.update_field.store(val, Ordering::SeqCst)
        }
    }
}

impl DigitalInputMonitor {
    pub fn new(input: Box<DigitalInput>, update_field: Arc<AtomicBool>, default: bool) -> Self {
        Self {
            input,
            update_field,
            default,
        }
    }
}