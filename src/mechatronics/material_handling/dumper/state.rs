use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub const DUMPER_RUNNING: usize = 1;
pub const DUMPER_RESETTING: usize = 2;
pub const DUMPER_STOPPED: usize = 0;

pub struct GlobalDumperState {
    action: AtomicUsize,
    enabled: AtomicBool,
}

impl GlobalDumperState {
    pub fn new() -> Self {
        Self {
            action: AtomicUsize::new(DUMPER_STOPPED),
            enabled: AtomicBool::new(false),
        }
    }

    pub fn get_current_state(&self) -> DumperStateInstance {
        DumperStateInstance::new(
            self.action.load(Ordering::Relaxed),
            self.enabled.load(Ordering::Relaxed),
        )
    }

    pub fn set_action(&self, action: usize) {
        self.action.store(action, Ordering::Relaxed)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed)
    }

    pub fn get_action(&self) -> usize {
        self.action.load(Ordering::Relaxed)
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

#[derive(Serialize)]
pub struct DumperStateInstance {
    action: usize,
    enabled: bool,
}

impl DumperStateInstance {
    fn new(action: usize, enabled: bool) -> Self {
        Self {
            action,
            enabled,
        }
    }

    pub fn get_action(&self) -> usize {
        self.action
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}