use crate::devices::motor_controllers::GlobalMotorState;
use crate::devices::motor_controllers::MotorStateInstance;

pub struct GlobalMotorGroupState<'a> {
    motors: Vec<&'a GlobalMotorState>,
}

impl<'a> GlobalMotorGroupState<'a> {
    pub fn new(motors: Vec<&'a GlobalMotorState>) -> Self {
        Self {
            motors,
        }
    }

    pub fn get_current_state(&self) -> MotorGroupStateInstance {
        MotorGroupStateInstance::new(
            self.motors
                .iter()
                .map(|m| m.get_current_state())
                .collect())
    }
}

#[derive(Serialize)]
pub struct MotorGroupStateInstance {
    motors: Vec<MotorStateInstance>,
}

impl MotorGroupStateInstance {
    pub fn new(motors: Vec<MotorStateInstance>) -> Self {
        Self {
            motors
        }
    }

    pub fn get_motors(&self) -> &Vec<MotorStateInstance> {
        &self.motors
    }
}