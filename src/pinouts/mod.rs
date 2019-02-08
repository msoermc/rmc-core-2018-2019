use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub mod sysfs_pin_wrappers;
pub mod sysfs_pwm_wrappers;

/// Runs a bash script which will enable the PWM drivers and configure the pins used by the program.
pub fn enable_pins() -> Result<(), ()> {
    let command_result = Command::new("sh").arg("enable-pwm.sh").output();

    match command_result {
        Ok(_) => Ok(()),
        Err(_) => {
            error!("Failed to enable the pwms!");
            Err(())
        }
    }
}

pub trait DigitalInput {
    fn get_value(&self) -> Option<bool>;
}

pub trait DigitalOutput: Send {
    fn set_value(&mut self, val: bool) -> Result<(), String>;
}

pub trait AnalogOutput: Send {
    fn set_value(&mut self, val: f32) -> Result<(), String>;
}

pub struct TestPin {
    state: Arc<AtomicBool>,
}

impl DigitalOutput for TestPin {
    fn set_value(&mut self, val: bool) -> Result<(), String> {
        self.state.swap(val, Ordering::SeqCst);
        Ok(())
    }
}

impl TestPin {
    pub fn new(state: Arc<AtomicBool>) -> Self {
        Self {
            state
        }
    }
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

impl TestPwm {
    pub fn new(state: Arc<atomic::Atomic<f32>>) -> Self {
        Self {
            state
        }
    }
}