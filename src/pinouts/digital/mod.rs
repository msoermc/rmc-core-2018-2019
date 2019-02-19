use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use crate::pinouts::digital::output::DigitalOutput;
use std::sync::atomic::Ordering;

pub mod input;

pub mod output;

pub struct TestPin {
    state: Arc<AtomicBool>,
}

impl TestPin {
    pub fn new(state: Arc<AtomicBool>) -> Self {
        Self {
            state
        }
    }
}

impl DigitalOutput for TestPin {
    fn set_value(&mut self, val: bool) -> Result<(), String> {
        self.state.swap(val, Ordering::SeqCst);
        Ok(())
    }
}