use std::sync::Arc;

use crate::motor_controllers::GlobalMotorState;
use crate::motor_controllers::MotorStateInstance;

pub struct GlobalLadderState {
    motor: Arc<GlobalMotorState>,
}

impl GlobalLadderState {
    pub fn new() -> Self {
        Self {
            motor: Arc::new(GlobalMotorState::new()),
        }
    }

    pub fn get_motor(&self) -> Arc<GlobalMotorState> {
        self.motor.clone()
    }

    pub fn get_current_state(&self) -> LadderStateInstance {
        LadderStateInstance::new(self.motor.get_current_state())
    }
}

#[derive(Serialize)]
pub struct LadderStateInstance {
    motor: MotorStateInstance,
}

impl LadderStateInstance {
    fn new(motor: MotorStateInstance) -> Self {
        Self {
            motor
        }
    }

    pub fn get_motor(&self) -> &MotorStateInstance {
        &self.motor
    }
}