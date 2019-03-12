use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::pinouts::digital::output::DigitalOutput;
use crate::pinouts::digital::input::DigitalInput;
use std::pin::Pin;

pub mod input;

pub mod output;

pub mod libbeaglebone;

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
    fn set_value(&mut self, val: bool) {
        self.state.swap(val, Ordering::SeqCst);
    }
}

impl DigitalInput for TestPin {
    fn get_value(&self) -> bool {
        self.state.load(Ordering::Relaxed)
    }
}