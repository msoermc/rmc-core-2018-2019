use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::mechatronics::bucket_ladder::state::limit::ActuatorStateInstance;
use crate::mechatronics::bucket_ladder::state::limit::GlobalLimitState;
use crate::mechatronics::bucket_ladder::state::ladder::GlobalLadderState;
use crate::mechatronics::bucket_ladder::state::ladder::LadderStateInstance;
use crate::mechatronics::bucket_ladder::state::actuator::GlobalActuatorState;

pub mod limit;
pub mod ladder;
pub mod actuator;

pub struct GlobalIntakeState {
    left_limit: Arc<GlobalLimitState>,
    right_limit: Arc<GlobalLimitState>,
    actuator: Arc<GlobalActuatorState>,
    ladder: Arc<GlobalLadderState>,
    enabled: AtomicBool,
}

impl GlobalIntakeState {
    pub fn new() -> Self {
        Self {
            left_limit: Arc::new(GlobalLimitState::new()),
            right_limit: Arc::new(GlobalLimitState::new()),
            actuator: Arc::new(GlobalActuatorState::new()),
            ladder: Arc::new(GlobalLadderState::new()),
            enabled: AtomicBool::new(false),
        }
    }

    pub fn get_current_state(&self) -> IntakeStateInstance {
        IntakeStateInstance::new(
            self.left_limit.get_current_state(),
            self.right_limit.get_current_state(),
            self.ladder.get_current_state(),
            self.enabled.load(Ordering::SeqCst),
        )
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn get_left_actuator(&self) -> Arc<GlobalLimitState> {
        self.left_limit.clone()
    }

    pub fn get_right_actuator(&self) -> Arc<GlobalLimitState> {
        self.right_limit.clone()
    }

    pub fn get_ladder(&self) -> Arc<GlobalLadderState> {
        self.ladder.clone()
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

#[derive(Serialize)]
pub struct IntakeStateInstance {
    left_limit: ActuatorStateInstance,
    right_limit: ActuatorStateInstance,
    ladder: LadderStateInstance,
    enabled: bool,
}

impl IntakeStateInstance {
    fn new(left_actuator: ActuatorStateInstance, right_actuator: ActuatorStateInstance,
           ladder: LadderStateInstance, enabled: bool) -> Self {
        Self {
            left_limit: left_actuator,
            right_limit: right_actuator,
            ladder,
            enabled,
        }
    }

    pub fn get_left_actuator(&self) -> &ActuatorStateInstance {
        &self.left_limit
    }

    pub fn get_right_actuator(&self) -> &ActuatorStateInstance {
        &self.right_limit
    }

    pub fn get_ladder(&self) -> &LadderStateInstance {
        &self.ladder
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}