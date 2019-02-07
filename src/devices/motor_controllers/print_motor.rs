use std::sync::Arc;

use super::*;

const FLOAT_ERROR: f32 = 0.05;

pub struct PrintMotor {
    name: String,
    state: Arc<GlobalMotorState>,
    is_stopped: bool,
}

impl MotorController for PrintMotor {
    fn set_speed(&mut self, new_speed: f32) {
        if (self.get_motor_state().get_speed() - new_speed < FLOAT_ERROR)
            || (new_speed - self.get_motor_state().get_speed() < FLOAT_ERROR) {
            info!("{}: -> {}", self.name, new_speed);
            self.get_motor_state().set_speed(new_speed);
        }

        self.is_stopped = false;
    }

    fn stop(&mut self) {
        if !self.is_stopped {
            info!("{}: STOP", self.name);
            self.is_stopped = true;
            self.get_motor_state().set_speed(0.0);
        }
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl PrintMotor {
    pub fn new(name: &str, state: Arc<GlobalMotorState>) -> PrintMotor {
        PrintMotor {
            name: name.to_string(),
            state,
            is_stopped: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::devices::motor_controllers::GlobalMotorState;

    use super::*;

    #[test]
    fn test_print_motor() {
        let state = Arc::new(GlobalMotorState::new());
        let mut motor = PrintMotor::new("t", state.clone());
        assert_eq!(0.0, motor.get_motor_state().get_speed());

        motor.set_speed(1.0);
        assert_eq!(1.0, motor.get_motor_state().get_speed());

        motor.set_speed(-1.0);
        assert_eq!(-1.0, motor.get_motor_state().get_speed());

        motor.stop();
        assert_eq!(0.0, motor.get_motor_state().get_speed());

        motor.set_speed(1.0);
        assert_eq!(1.0, motor.get_motor_state().get_speed());
    }
}