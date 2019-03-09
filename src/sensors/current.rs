use std::sync::Arc;

use atomic::Atomic;
use crate::pinouts::analog::input::AnalogInput;
use crate::framework::Runnable;

/// Monitors current and updates it's state accordingly.
pub struct CurrentMonitor {
    input: Box<AnalogInput>,
    current: Arc<Atomic<f32>>,
}

impl CurrentMonitor {
    pub fn new(input: Box<AnalogInput>, current: Arc<Atomic<f32>>) -> Self {
        Self {
            input,
            current,
        }
    }
}

impl Runnable for CurrentMonitor {
    fn init(&mut self) {
        unimplemented!()
    }

    fn run(&mut self) {
        unimplemented!()
    }
}