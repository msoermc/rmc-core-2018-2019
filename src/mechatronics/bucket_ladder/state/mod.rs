use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::mechatronics::bucket_ladder::state::actuator::ActuatorStateInstance;
use crate::mechatronics::bucket_ladder::state::actuator::GlobalActuatorState;
use crate::mechatronics::bucket_ladder::state::ladder::GlobalLadderState;
use crate::mechatronics::bucket_ladder::state::ladder::LadderStateInstance;

pub mod actuator;
pub mod ladder;

#[derive(Default)]
pub struct GlobalIntakeState {
    left_actuator: Arc<GlobalActuatorState>,
    right_actuator: Arc<GlobalActuatorState>,
    ladder: Arc<GlobalLadderState>,
    enabled: AtomicBool,
}

impl GlobalIntakeState {
    pub fn new() -> Self {
        Self {
            left_actuator: Arc::new(GlobalActuatorState::new()),
            right_actuator: Arc::new(GlobalActuatorState::new()),
            ladder: Arc::new(GlobalLadderState::new()),
            enabled: AtomicBool::new(false),
        }
    }

    pub fn get_current_state(&self) -> IntakeStateInstance {
        IntakeStateInstance::new(
            self.left_actuator.get_current_state(),
            self.right_actuator.get_current_state(),
            self.ladder.get_current_state(),
            self.enabled.load(Ordering::SeqCst),
        )
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn get_left_actuator(&self) -> Arc<GlobalActuatorState> {
        self.left_actuator.clone()
    }

    pub fn get_right_actuator(&self) -> Arc<GlobalActuatorState> {
        self.right_actuator.clone()
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
    left_actuator: ActuatorStateInstance,
    right_actuator: ActuatorStateInstance,
    ladder: LadderStateInstance,
    enabled: bool,
}

impl IntakeStateInstance {
    fn new(left_actuator: ActuatorStateInstance, right_actuator: ActuatorStateInstance,
           ladder: LadderStateInstance, enabled: bool) -> Self {
        Self {
            left_actuator,
            right_actuator,
            ladder,
            enabled,
        }
    }

    pub fn get_left_actuator(&self) -> &ActuatorStateInstance {
        &self.left_actuator
    }

    pub fn get_right_actuator(&self) -> &ActuatorStateInstance {
        &self.right_actuator
    }

    pub fn get_ladder(&self) -> &LadderStateInstance {
        &self.ladder
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
}