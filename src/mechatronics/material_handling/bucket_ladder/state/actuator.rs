use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct GlobalActuatorState {
    upper: AtomicBool,
    lower: AtomicBool,
}

impl GlobalActuatorState {
    pub fn new() -> Self {
        GlobalActuatorState {
            upper: AtomicBool::new(false),
            lower: AtomicBool::new(false),
        }
    }

    pub fn get_current_state(&self) -> ActuatorStateInstance {
        ActuatorStateInstance::new(
            self.upper.load(Ordering::Relaxed),
            self.lower.load(Ordering::Relaxed),
        )
    }

    pub fn set_upper(&self, upper: bool) {
        self.upper.store(upper, Ordering::Relaxed);
    }

    pub fn set_lower(&self, lower: bool) {
        self.lower.store(lower, Ordering::Relaxed);
    }

    pub fn get_upper(&self) -> bool {
        self.upper.load(Ordering::Relaxed)
    }

    pub fn get_lower(&self) -> bool {
        self.lower.load(Ordering::Relaxed)
    }
}

#[derive(Serialize)]
pub struct ActuatorStateInstance {
    upper: bool,
    lower: bool,
}

impl ActuatorStateInstance {
    fn new(upper: bool, lower: bool) -> Self {
        ActuatorStateInstance {
            upper,
            lower,
        }
    }
}