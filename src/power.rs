use std::sync::Arc;

use atomic::Atomic;

use crate::pinouts::AnalogInput;

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

    /// Launches the power monitor, taking over this thread.
    pub fn launch(mut self) {
        // TODO
    }
}