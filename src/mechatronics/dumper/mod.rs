use std::sync::Arc;

use crate::mechatronics::dumper::state::GlobalDumperState;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;
use crate::devices::motor_controllers::MotorController;

#[cfg(test)]
mod tests;

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