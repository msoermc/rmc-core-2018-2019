use std::sync::Arc;

use crate::mechatronics::dumper::state::GlobalDumperState;
use crate::motor_controllers::MotorController;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;

pub mod state;

pub struct Dumper {
    motors: Box<MotorController>,
    state: Arc<GlobalDumperState>,
    life: Arc<GlobalLifeState>,
}

impl Dumper {
    pub fn new(life: Arc<GlobalLifeState>, motors: Box<MotorController>, state: Arc<GlobalDumperState>) -> Self {
        Self {
            motors,
            state,
            life,
        }
    }

    pub fn enable(&mut self) {
        self.state.set_enabled(true);
    }

    pub fn disable(&mut self) {
        self.state.set_enabled(false);
        self.stop();
    }

    pub fn dump(&mut self) {
        if self.state.get_enabled() && self.life.is_alive() {
            self.motors.set_speed(DUMPING_RATE);
        } else {
            self.stop();
        }
    }

    pub fn reset(&mut self) {
        if self.state.get_enabled() && self.life.is_alive() {
            self.motors.set_speed(DUMPER_RESET_RATE);
        } else {
            self.stop();
        }
    }

    pub fn stop(&mut self) {
        self.motors.stop();
    }

    pub fn run_cycle(&mut self) {
        if self.life.is_alive() {
            // TODO;
        } else {
            self.stop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::motor_controllers::test_motor::TestMotor;

    #[test]
    fn test_setup() {
        let life = Arc::new(GlobalLifeState::new());
        let state = Arc::new(GlobalDumperState::new());
        let motor = Box::new(TestMotor::new(state.get_motor()));

        let _dumper = Dumper::new(life.clone(), motor, state.clone());

        assert_eq!(0.0, state.get_motor().get_speed());
        assert_eq!(false, state.get_enabled());
    }

    #[test]
    fn test_dump() {
        let life = Arc::new(GlobalLifeState::new());
        let state = Arc::new(GlobalDumperState::new());
        let motor = Box::new(TestMotor::new(state.get_motor()));

        let mut dumper = Dumper::new(life.clone(), motor, state.clone());
        state.set_enabled(true);

        dumper.dump();
        assert_eq!(DUMPING_RATE, state.get_motor().get_speed());
        dumper.run_cycle();
        assert_eq!(DUMPING_RATE, state.get_motor().get_speed());
    }

    #[test]
    fn test_reset() {
        let life = Arc::new(GlobalLifeState::new());
        let state = Arc::new(GlobalDumperState::new());
        let motor = Box::new(TestMotor::new(state.get_motor()));

        let mut dumper = Dumper::new(life.clone(), motor, state.clone());
        state.set_enabled(true);

        dumper.reset();
        assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
        dumper.run_cycle();
        assert_eq!(DUMPER_RESET_RATE, state.get_motor().get_speed());
    }
}