use std::sync::Arc;
use atomic::{Ordering, Atomic};

pub mod pwm;

pub trait AnalogOutput: Send {
    fn set_value(&mut self, val: f32);
}


pub trait PwmOutput: AnalogOutput {
    fn set_pulse_duty_cycle(&mut self, val: u32);
    fn set_period(&mut self, val: u32);
}


pub struct TestPwm {
    value: Arc<atomic::Atomic<f32>>,
    duty_cycle: Arc<Atomic<usize>>,
    period: Arc<Atomic<usize>>,
}

impl AnalogOutput for TestPwm {
    fn set_value(&mut self, val: f32) {
        self.value.swap(val, atomic::Ordering::SeqCst);
        self.duty_cycle.swap((val * self.period.load(Ordering::SeqCst) as f32) as usize, Ordering::SeqCst);
    }
}

impl PwmOutput for TestPwm {
    fn set_pulse_duty_cycle(&mut self, val: u32) {
        self.duty_cycle.store(val as usize, Ordering::SeqCst);
    }

    fn set_period(&mut self, val: u32) {
        self.period.store(val as usize, Ordering::SeqCst);
    }
}

impl TestPwm {
    pub fn new(state: Arc<atomic::Atomic<f32>>) -> Self {
        Self {
            value: state,
            duty_cycle: Arc::new(Atomic::new(0)),
            period: Arc::new(Atomic::new(20_000)),
        }
    }

    pub fn pwm(value: Arc<atomic::Atomic<f32>>, duty_cycle: Arc<Atomic<usize>>, period: Arc<Atomic<usize>>) -> Self {
        Self {
            value,
            duty_cycle,
            period,
        }
    }
}