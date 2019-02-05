use std::sync::atomic::AtomicBool;

use crate::mechatronics::material_handling::bucket_ladder::state::actuator::ActuatorStateInstance;
use crate::mechatronics::material_handling::bucket_ladder::state::actuator::GlobalActuatorState;
use crate::mechatronics::material_handling::bucket_ladder::state::ladder::GlobalLadderState;
use crate::mechatronics::material_handling::bucket_ladder::state::ladder::LadderStateInstance;
use std::sync::atomic::Ordering;

pub mod actuator;
pub mod ladder;

pub struct GlobalIntakeState {
    left_actuator: GlobalActuatorState,
    right_actuator: GlobalActuatorState,
    ladder: GlobalLadderState,
    enabled: AtomicBool,
}

impl GlobalIntakeState {
    pub fn new(left_actuator: GlobalActuatorState, right_actuator: GlobalActuatorState,
               ladder: GlobalLadderState, enabled: AtomicBool) -> Self {
        Self {
            left_actuator,
            right_actuator,
            ladder,
            enabled
        }
    }

    pub fn get_current_state(&self) -> IntakeStateInstance {
        IntakeStateInstance::new(
            self.left_actuator.get_current_state(),
            self.right_actuator.get_current_state(),
            self.ladder.get_current_state(),
            self.enabled.load(Ordering::Relaxed),
        )
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn get_left_actuator(&self) -> &GlobalActuatorState {
        &self.left_actuator
    }

    pub fn get_right_actuator(&self) -> &GlobalActuatorState {
        &self.right_actuator
    }

    pub fn get_ladder(&self) -> &GlobalLadderState {
        &self.ladder
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

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