use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct GlobalDumperState {
    enabled: AtomicBool,
}

impl GlobalDumperState {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
        }
    }

    pub fn get_current_state(&self) -> DumperStateInstance {
        DumperStateInstance::new(
            self.enabled.load(Ordering::Relaxed),
        )
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed)
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

#[derive(Serialize)]
pub struct DumperStateInstance {
    enabled: bool,
}

impl DumperStateInstance {
    fn new(enabled: bool) -> Self {
        Self {
            enabled,
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}