use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub const ACTUATOR_RISING: usize = 2;
pub const ACTUATOR_LOWERING: usize = 1;
pub const ACTUATOR_STOPPED: usize = 0;

pub struct GlobalActuatorState {
    upper: AtomicBool,
    lower: AtomicBool,
    action: AtomicUsize,
}

impl GlobalActuatorState {
    pub fn new() -> Self {
        GlobalActuatorState {
            upper: AtomicBool::new(false),
            lower: AtomicBool::new(false),
            action: AtomicUsize::new(ACTUATOR_STOPPED),
        }
    }

    pub fn get_current_state(&self) -> ActuatorStateInstance {
        ActuatorStateInstance::new(
            self.upper.load(Ordering::Relaxed),
            self.lower.load(Ordering::Relaxed),
            self.action.load(Ordering::Relaxed),
        )
    }

    pub fn set_upper(&self, upper: bool) {
        self.upper.store(upper, Ordering::Relaxed);
    }

    pub fn set_lower(&self, lower: bool) {
        self.lower.store(lower, Ordering::Relaxed);
    }

    pub fn set_action(&self, action: usize) {
        self.action.store(action, Ordering::Relaxed);
    }

    pub fn get_upper(&self) -> bool {
        self.upper.load(Ordering::Relaxed)
    }

    pub fn get_lower(&self) -> bool {
        self.lower.load(Ordering::Relaxed)
    }

    pub fn get_action(&self) -> usize {
        self.action.load(Ordering::Relaxed)
    }
}

#[derive(Serialize)]
pub struct ActuatorStateInstance {
    upper: bool,
    lower: bool,
    action: usize,
}

impl ActuatorStateInstance {
    fn new(upper: bool, lower: bool, action: usize) -> Self {
        ActuatorStateInstance {
            upper,
            lower,
            action,
        }
    }

    pub fn get_upper(&self) -> bool {
        self.upper
    }

    pub fn get_lower(&self) -> bool {
        self.lower
    }

    pub fn get_action(&self) -> usize {
        self.action
    }
}