use atomic::Ordering;
use std::sync::Arc;

pub trait AnalogOutput: Send {
    fn set_value(&mut self, val: f32) -> Result<(), String>;
}


pub trait PwmOutput: AnalogOutput {
    fn set_pulse_width(&mut self, val: f32) -> Result<(), String>;
}


pub struct TestPwm {
    state: Arc<atomic::Atomic<f32>>,
}

impl AnalogOutput for TestPwm {
    fn set_value(&mut self, val: f32) -> Result<(), String> {
        self.state.swap(val, atomic::Ordering::SeqCst);
        Ok(())
    }
}

impl PwmOutput for TestPwm {
    fn set_pulse_width(&mut self, val: f32) -> Result<(), String> {
        self.state.swap(val, Ordering::SeqCst);
        Ok(())
    }
}

impl TestPwm {
    pub fn new(state: Arc<atomic::Atomic<f32>>) -> Self {
        Self {
            state
        }
    }
}