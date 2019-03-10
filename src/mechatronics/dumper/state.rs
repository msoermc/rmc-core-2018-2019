use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::motor_controllers::GlobalMotorState;
use crate::motor_controllers::MotorStateInstance;

pub struct GlobalDumperState {
    enabled: AtomicBool,
    motor: Arc<GlobalMotorState>,
    upper_limit: Arc<AtomicBool>,
    lower_limit: Arc<AtomicBool>,
}

impl GlobalDumperState {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            motor: Arc::new(GlobalMotorState::new()),
            upper_limit: Arc::new(AtomicBool::new(false)),
            lower_limit: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_current_state(&self) -> DumperStateInstance {
        DumperStateInstance::new(
            self.enabled.load(Ordering::Relaxed),
            self.motor.get_current_state(),
            self.upper_limit.load(Ordering::Relaxed),
            self.lower_limit.load(Ordering::Relaxed),
        )
    }

    pub fn get_motor(&self) -> Arc<GlobalMotorState> {
        self.motor.clone()
    }

    pub fn get_upper_limit(&self) -> Arc<AtomicBool> {
        self.upper_limit.clone()
    }

    pub fn get_lower_limit(&self) -> Arc<AtomicBool> {
        self.lower_limit.clone()
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
    upper_limit: bool,
    lower_limit: bool,
}

impl DumperStateInstance {
    fn new(enabled: bool, motor: MotorStateInstance, upper_limit: bool, lower_limit: bool) -> Self {
        Self {
            enabled,
            motor,
            upper_limit,
            lower_limit,
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_motor(&self) -> &MotorStateInstance {
        &self.motor
    }
}