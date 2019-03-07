use std::sync::Arc;

use crate::motor_controllers::GlobalMotorState;
use crate::motor_controllers::MotorStateInstance;

pub struct GlobalActuatorState {
    motor: Arc<GlobalMotorState>,
}

impl GlobalActuatorState {
    pub fn new() -> Self {
        GlobalActuatorState {
            motor: Arc::new(GlobalMotorState::new()),
        }
    }

    pub fn get_current_state(&self) -> ActuatorStateInstance {
        ActuatorStateInstance::new(
            self.motor.get_current_state(),
        )
    }

    pub fn get_motor(&self) -> Arc<GlobalMotorState> {
        self.motor.clone()
    }
}

#[derive(Serialize)]
pub struct ActuatorStateInstance {
    motor: MotorStateInstance,
}

impl ActuatorStateInstance {
    fn new(motor: MotorStateInstance) -> Self {
        ActuatorStateInstance {
            motor,
        }
    }

    pub fn get_motor(&self) -> &MotorStateInstance {
        &self.motor
    }
}