use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::devices::motor_controllers::GlobalMotorState;
use crate::devices::motor_controllers::MotorStateInstance;

pub struct GlobalDriveTrainState {
    enabled: AtomicBool,
    front_left: Arc<GlobalMotorState>,
    rear_left: Arc<GlobalMotorState>,
    front_right: Arc<GlobalMotorState>,
    rear_right: Arc<GlobalMotorState>,
}

impl GlobalDriveTrainState {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            front_left: Arc::new(GlobalMotorState::new()),
            rear_left: Arc::new(GlobalMotorState::new()),
            front_right: Arc::new(GlobalMotorState::new()),
            rear_right: Arc::new(GlobalMotorState::new()),
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed)
    }

    pub fn get_left(&self) -> Arc<GlobalMotorState> {
        self.front_left.clone()
    }

    pub fn get_right(&self) -> Arc<GlobalMotorState> {
        self.front_right.clone()
    }

    pub fn get_rear_left(&self) -> Arc<GlobalMotorState> {
        self.rear_left.clone()
    }

    pub fn get_rear_right(&self) -> Arc<GlobalMotorState> {
        self.rear_right.clone()
    }

    pub fn get_current_state(&self) -> DriveTrainStateInstance {
        DriveTrainStateInstance::new(
            self.enabled.load(Ordering::Relaxed),
            self.front_left.get_current_state(),
            self.front_right.get_current_state(),
            self.rear_left.get_current_state(),
            self.rear_right.get_current_state(),
        )
    }
}

#[derive(Serialize)]
pub struct DriveTrainStateInstance {
    enabled: bool,
    front_left: MotorStateInstance,
    front_right: MotorStateInstance,
    rear_left: MotorStateInstance,
    rear_right: MotorStateInstance,
}

impl DriveTrainStateInstance {
    fn new(enabled: bool, front_left: MotorStateInstance, front_right: MotorStateInstance,
           rear_left: MotorStateInstance, rear_right: MotorStateInstance) -> Self {
        Self {
            enabled,
            front_left,
            front_right,
            rear_left,
            rear_right,
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}