use std::sync::Arc;

use atomic::Ordering;

pub mod pwm;

pub trait AnalogOutput: Send {
    fn set_value(&mut self, val: f32);
}


pub trait PwmOutput: AnalogOutput {
    fn set_pulse_duty_cycle(&mut self, val: u32);
    fn set_period(&mut self, val: u32);
}


pub struct TestPwm {
    state: Arc<atomic::Atomic<f32>>,
}

impl AnalogOutput for TestPwm {
    fn set_value(&mut self, val: f32) {
        self.state.swap(val, atomic::Ordering::SeqCst);
    }
}

impl PwmOutput for TestPwm {
    fn set_pulse_duty_cycle(&mut self, val: u32) {
        unimplemented!()
    }

    fn set_period(&mut self, val: u32) {
        unimplemented!()
    }
}

impl TestPwm {
    pub fn new(state: Arc<atomic::Atomic<f32>>) -> Self {
        Self {
            state
        }
    }
}