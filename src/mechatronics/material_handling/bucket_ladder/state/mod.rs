use crate::mechatronics::material_handling::bucket_ladder::state::actuator::ActuatorStateInstance;
use crate::mechatronics::material_handling::bucket_ladder::state::actuator::GlobalActuatorState;
use crate::mechatronics::material_handling::bucket_ladder::state::ladder::GlobalLadderState;
use crate::mechatronics::material_handling::bucket_ladder::state::ladder::LadderStateInstance;

pub mod actuator;
pub mod ladder;

pub struct GlobalIntakeState {
    left_actuator: GlobalActuatorState,
    right_actuator: GlobalActuatorState,
    ladder: GlobalLadderState,
}

impl GlobalIntakeState {
    pub fn new(left_actuator: GlobalActuatorState, right_actuator: GlobalActuatorState, ladder: GlobalLadderState) -> Self {
        Self {
            left_actuator,
            right_actuator,
            ladder,
        }
    }

    pub fn get_current_state(&self) -> IntakeStateInstance {
        IntakeStateInstance::new(
            self.left_actuator.get_current_state(),
            self.right_actuator.get_current_state(),
            self.ladder.get_current_state(),
        )
    }
}

pub struct IntakeStateInstance {
    left_actuator: ActuatorStateInstance,
    right_actuator: ActuatorStateInstance,
    ladder: LadderStateInstance,
}

impl IntakeStateInstance {
    fn new(left_actuator: ActuatorStateInstance, right_actuator: ActuatorStateInstance,
           ladder: LadderStateInstance) -> Self {
        Self {
            left_actuator,
            right_actuator,
            ladder,
        }
    }
}