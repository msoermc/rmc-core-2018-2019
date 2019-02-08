use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::motor_controllers::GlobalMotorState;
use crate::motor_controllers::MotorStateInstance;

pub struct GlobalDriveTrainState {
    enabled: AtomicBool,
    left: Arc<GlobalMotorState>,
    right: Arc<GlobalMotorState>,
}

impl GlobalDriveTrainState {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            left: Arc::new(GlobalMotorState::new()),
            right: Arc::new(GlobalMotorState::new()),
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed)
    }

    pub fn get_left(&self) -> Arc<GlobalMotorState> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Arc<GlobalMotorState> {
        self.right.clone()
    }

    pub fn get_current_state(&self) -> DriveTrainStateInstance {
        DriveTrainStateInstance::new(
            self.enabled.load(Ordering::Relaxed),
            self.left.get_current_state(),
            self.right.get_current_state(),
        )
    }
}

#[derive(Serialize)]
pub struct DriveTrainStateInstance {
    enabled: bool,
    left: MotorStateInstance,
    right: MotorStateInstance,
}

impl DriveTrainStateInstance {
    fn new(enabled: bool, left: MotorStateInstance, right: MotorStateInstance) -> Self {
        Self {
            enabled,
            left,
            right,
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}