use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;
use crate::mechatronics::material_handling::dumper::state::GlobalDumperState;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub mod state;

pub struct Dumper {
    motors: MotorGroup,
    state: Arc<GlobalDumperState>,
    life: Arc<GlobalLifeState>,
}

impl Dumper {
    pub fn new(life: Arc<GlobalLifeState>, motors: MotorGroup, state: Arc<GlobalDumperState>) -> Self {
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
        self.motors.maintain_last();
    }
}