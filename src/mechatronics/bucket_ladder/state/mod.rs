use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::mechatronics::bucket_ladder::state::actuator::ActuatorLimitStateInstance;
use crate::mechatronics::bucket_ladder::state::actuator::GlobalActuatorLimitState;
use crate::motor_controllers::{GlobalMotorState, MotorStateInstance};

pub mod actuator;

pub struct GlobalIntakeState {
    left_limits: Arc<GlobalActuatorLimitState>,
    right_limits: Arc<GlobalActuatorLimitState>,
    actuator: Arc<GlobalMotorState>,
    digger: Arc<GlobalMotorState>,
    enabled: AtomicBool,
}

impl GlobalIntakeState {
    pub fn new() -> Self {
        Self {
            left_limits: Arc::new(GlobalActuatorLimitState::new()),
            right_limits: Arc::new(GlobalActuatorLimitState::new()),
            actuator: Arc::new(GlobalMotorState::new()),
            digger: Arc::new(GlobalMotorState::new()),
            enabled: AtomicBool::new(false),
        }
    }

    pub fn get_current_state(&self) -> IntakeStateInstance {
        IntakeStateInstance::new(
            self.left_limits.get_current_state(),
            self.right_limits.get_current_state(),
            self.actuator.get_current_state(),
            self.digger.get_current_state(),
            self.enabled.load(Ordering::SeqCst),
        )
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn get_left_actuator(&self) -> Arc<GlobalActuatorLimitState> {
        self.left_limits.clone()
    }

    pub fn get_right_actuator(&self) -> Arc<GlobalActuatorLimitState> {
        self.right_limits.clone()
    }

    pub fn get_digger(&self) -> Arc<GlobalMotorState> {
        self.digger.clone()
    }

    pub fn get_actuator(&self) -> Arc<GlobalMotorState> {
        self.actuator.clone()
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

#[derive(Serialize)]
pub struct IntakeStateInstance {
    left_limits: ActuatorLimitStateInstance,
    right_limits: ActuatorLimitStateInstance,
    actuator: MotorStateInstance,
    digger: MotorStateInstance,
    enabled: bool,
}

impl IntakeStateInstance {
    fn new(left_limits: ActuatorLimitStateInstance, right_limits: ActuatorLimitStateInstance, actuator: MotorStateInstance, digger: MotorStateInstance, enabled: bool) -> Self {
        Self {
            left_limits,
            right_limits,
            actuator,
            digger,
            enabled,
        }
    }

    pub fn get_left_actuator(&self) -> &ActuatorLimitStateInstance {
        &self.left_limits
    }

    pub fn get_right_actuator(&self) -> &ActuatorLimitStateInstance {
        &self.right_limits
    }

    pub fn get_actuator(&self) -> &MotorStateInstance {
        &self.actuator
    }

    pub fn get_digger(&self) -> &MotorStateInstance {
        &self.digger
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}