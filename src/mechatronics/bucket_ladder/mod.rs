use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use atomic::Ordering;

use crate::mechatronics::bucket_ladder::state::GlobalIntakeState;
use crate::motor_controllers::MotorController;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;

pub mod state;

#[cfg(test)]
mod tests;

pub struct Intake {
    actuator: Box<MotorController>,
    ladder: Box<MotorController>,
    state: Arc<GlobalIntakeState>,
    life: Arc<GlobalLifeState>,
    enabled_cache: bool,
}

impl Intake {
    pub fn new(ladder: Box<MotorController>, actuator: Box<MotorController>, state: Arc<GlobalIntakeState>, life: Arc<GlobalLifeState>) -> Self {
        let enabled_cache = state.get_enabled();
        Self {
            actuator,
            ladder,
            state,
            life,
            enabled_cache,
        }
    }

    pub fn enable(&mut self) {
        self.state.set_enabled(true);
        self.enabled_cache = true;
    }

    pub fn disable(&mut self) {
        self.state.set_enabled(false);
        self.enabled_cache = false;
        self.stop_ladder();
        self.stop_actuators();
    }

    pub fn raise(&mut self) {
        if self.check_if_safe_to_move_actuators(self.state.get_left_actuator().get_upper(), self.state.get_right_actuator().get_upper()) {
            self.actuator.set_speed(MH_ACTUATOR_RATE);
        } else {
            self.stop_actuators()
        }
    }

    pub fn lower(&mut self) {
        if self.check_if_safe_to_move_actuators(self.state.get_left_actuator().get_lower(), self.state.get_right_actuator().get_lower()) {
            self.actuator.set_speed(-MH_ACTUATOR_RATE);
        } else {
            self.stop_actuators();
        }
    }

    pub fn stop_actuators(&mut self) {
        self.actuator.stop();
    }

    pub fn dig(&mut self) {
        if self.is_enabled() && self.life.is_alive() {
            self.ladder.set_speed(DIGGING_RATE);
        } else {
            self.stop_ladder();
        }
    }

    pub fn stop_ladder(&mut self) {
        self.ladder.stop();
    }

    pub fn run_cycle(&mut self) {}

    #[inline]
    fn is_enabled(&self) -> bool {
        self.enabled_cache
    }

    #[inline]
    fn check_if_safe_to_move_actuators(&self, left: Arc<AtomicBool>, right: Arc<AtomicBool>) -> bool {
        self.is_enabled() && self.life.is_alive() && check_actuator_limits(left, right)
    }
}

#[inline]
fn check_actuator_limits(left: Arc<AtomicBool>, right: Arc<AtomicBool>) -> bool {
    !(left.load(Ordering::SeqCst) || right.load(Ordering::SeqCst))
}