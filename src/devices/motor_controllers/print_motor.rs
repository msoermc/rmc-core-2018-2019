use super::*;

const FLOAT_ERROR: f32 = 0.05;

pub struct PrintMotor {
    name: String,
    inverted: bool,
    state: GlobalMotorState,
    last: f32,
    is_stopped: bool,
}

impl MotorController for PrintMotor {
    fn set_speed(&mut self, new_speed: f32) {
        if self.last - new_speed < FLOAT_ERROR && new_speed - self.last > FLOAT_ERROR {
            info!("{}: -> {}", self.name, new_speed);
            self.last = new_speed;
        }

        self.is_stopped = false;
    }

    fn stop(&mut self) {
        if !self.is_stopped {
            info!("{}: STOP", self.name);
            self.is_stopped = true;
        }
    }

    fn invert(&mut self) {
        info!("{}: INVERT", self.name);
        self.inverted = !self.inverted;
    }

    fn get_motor_state(&self) -> &GlobalMotorState {
        &self.state
    }
}

impl PrintMotor {
    pub fn new(name: &str) -> PrintMotor {
        PrintMotor {
            name: name.to_string(),
            inverted: false,
            state: GlobalMotorState::new(),
            last: -10.0,
            is_stopped: false,
        }
    }
}