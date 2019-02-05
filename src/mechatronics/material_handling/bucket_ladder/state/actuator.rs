use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub const ACTUATOR_RISING: u8 = 2;
pub const ACTUATOR_LOWERING: u8 = 1;
pub const ACTUATOR_STOPPED: u8 = 0;

pub struct GlobalActuatorState {
    upper: AtomicBool,
    lower: AtomicBool,
    enabled: AtomicBool,
    action: AtomicUsize,
}

impl GlobalActuatorState {
    pub fn new(upper: AtomicBool, lower: AtomicBool, enabled: AtomicBool, action: AtomicUsize) -> Self {
        GlobalActuatorState {
            upper,
            lower,
            enabled,
            action,
        }
    }

    pub fn get_current_state(&self) -> ActuatorStateInstance {
        ActuatorStateInstance::new(
            self.upper.load(Ordering::Relaxed),
            self.lower.load(Ordering::Relaxed),
            self.enabled.load(Ordering::Relaxed),
            self.action.load(Ordering::Relaxed),
        )
    }
}

#[derive(Serialize)]
pub struct ActuatorStateInstance {
    upper: bool,
    lower: bool,
    enabled: bool,
    action: usize,
}

impl ActuatorStateInstance {
    fn new(upper: bool, lower: bool, enabled: bool, action: usize) -> Self {
        ActuatorStateInstance {
            upper,
            lower,
            enabled,
            action,
        }
    }
}