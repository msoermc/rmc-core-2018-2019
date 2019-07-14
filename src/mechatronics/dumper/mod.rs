use std::sync::Arc;

use atomic::Ordering;

use crate::mechatronics::dumper::state::GlobalDumperState;
use crate::motor_controllers::MotorController;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;

pub mod state;

#[cfg(test)]
mod tests;

pub struct Dumper {
    motors: Box<dyn MotorController>,
    state: Arc<GlobalDumperState>,
    life: Arc<GlobalLifeState>,
    enabled: bool,
    action: DumperAction,
}

impl Dumper {
    pub fn new(life: Arc<GlobalLifeState>, motors: Box<dyn MotorController>, state: Arc<GlobalDumperState>) -> Self {
        let enabled = state.get_enabled();
        Self {
            motors,
            state,
            life,
            enabled,
            action: DumperAction::Stopped,
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
        if self.enabled && !self.state.get_upper_limit().load(Ordering::Relaxed) && self.life.is_alive() {
            self.motors.set_speed(DUMPING_RATE);
            self.action = DumperAction::Dumping;
        }
    }

    pub fn reset(&mut self) {
        if self.enabled && !self.state.get_lower_limit().load(Ordering::Relaxed) && self.life.is_alive() {
            self.motors.set_speed(DUMPER_RESET_RATE);
            self.action = DumperAction::Resetting;
        }
    }

    pub fn stop(&mut self) {
        self.motors.stop();
        self.action = DumperAction::Stopped
    }

    pub fn run_cycle(&mut self) {
        if self.enabled {
            match self.action {
                DumperAction::Dumping => {
                    if self.state.get_upper_limit().load(Ordering::Relaxed) {
                        self.stop();
                    }
                },
                DumperAction::Resetting => {
                    if self.state.get_lower_limit().load(Ordering::Relaxed) {
                        self.stop();
                    }
                },
                DumperAction::Stopped => {},
            }
        }
    }
}

enum DumperAction {
    Dumping,
    Resetting,
    Stopped,
}