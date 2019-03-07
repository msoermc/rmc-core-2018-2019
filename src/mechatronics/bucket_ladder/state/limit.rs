use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct GlobalLimitState {
    upper: Arc<AtomicBool>,
    lower: Arc<AtomicBool>,
}

impl GlobalLimitState {
    pub fn new() -> Self {
        GlobalLimitState {
            upper: Arc::new(AtomicBool::new(false)),
            lower: Arc::new(AtomicBool::new(false)),
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

    pub fn get_upper(&self) -> Arc<AtomicBool> {
        self.upper.clone()
    }

    pub fn get_lower(&self) -> Arc<AtomicBool> {
        self.lower.clone()
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

    pub fn get_upper(&self) -> bool {
        self.upper
    }

    pub fn get_lower(&self) -> bool {
        self.lower
    }
}