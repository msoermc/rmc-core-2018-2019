use std::sync::Arc;

use crate::framework::Runnable;
use crate::pinouts::analog::input::AnalogInput;
use crate::status::current::GlobalCurrentState;

/// Monitors current and updates it's state accordingly.
pub struct CurrentMonitor {
    input: Box<AnalogInput>,
    current: Arc<GlobalCurrentState>,
}

impl CurrentMonitor {
    pub fn new(input: Box<AnalogInput>, current: Arc<GlobalCurrentState>) -> Self {
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