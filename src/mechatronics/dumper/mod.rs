use std::sync::Arc;

use crate::mechatronics::dumper::state::GlobalDumperState;
use crate::motor_controllers::MotorController;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;

pub mod state;

#[cfg(test)]
mod tests;

pub struct Dumper {
    motors: Box<MotorController>,
    state: Arc<GlobalDumperState>,
    life: Arc<GlobalLifeState>,
    enabled: bool,
}

impl Dumper {
    pub fn new(life: Arc<GlobalLifeState>, motors: Box<MotorController>, state: Arc<GlobalDumperState>) -> Self {
        let enabled = state.get_enabled();
        Self {
            motors,
            state,
            life,
            enabled
        }
    }

    pub fn enable(&mut self) {
        self.state.set_enabled(true);
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.state.set_enabled(false);
        self.enabled = false;
        self.stop();
    }

    pub fn dump(&mut self) {
        if self.enabled && self.life.is_alive() {
            self.motors.set_speed(DUMPING_RATE);
        } else {
            self.stop();
        }
    }

    pub fn reset(&mut self) {
        if self.enabled && self.life.is_alive() {
            self.motors.set_speed(DUMPER_RESET_RATE);
        } else {
            self.stop();
        }
    }

    pub fn stop(&mut self) {
        self.motors.stop();
    }

    pub fn run_cycle(&mut self) {
        if self.enabled && self.life.is_alive() {
            // TODO;
        } else if self.enabled {
            self.stop();
        }
    }
}