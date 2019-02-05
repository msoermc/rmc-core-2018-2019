use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct GlobalDriveTrainState {
    enabled: AtomicBool
}

impl GlobalDriveTrainState {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed)
    }

    pub fn get_current_state(&self) -> DriveTrainStateInstance {
        DriveTrainStateInstance::new(self.enabled.load(Ordering::Relaxed))
    }
}

#[derive(Serialize)]
pub struct DriveTrainStateInstance {
    enabled: bool
}

impl DriveTrainStateInstance {
    fn new(enabled: bool) -> Self {
        Self {
            enabled,
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}