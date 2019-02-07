use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::devices::motor_controllers::GlobalMotorState;
use crate::devices::motor_controllers::MotorStateInstance;

pub struct GlobalDumperState {
    enabled: AtomicBool,
    motor: Arc<GlobalMotorState>,
}

impl GlobalDumperState {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            motor: Arc::new(GlobalMotorState::new()),
        }
    }

    pub fn get_current_state(&self) -> DumperStateInstance {
        DumperStateInstance::new(
            self.enabled.load(Ordering::Relaxed),
            self.motor.get_current_state(),
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
    motor: MotorStateInstance,
}

impl DumperStateInstance {
    fn new(enabled: bool, motor: MotorStateInstance) -> Self {
        Self {
            enabled,
            motor,
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}