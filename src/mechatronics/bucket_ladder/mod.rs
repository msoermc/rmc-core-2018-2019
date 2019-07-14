use std::sync::Arc;

use crate::mechatronics::bucket_ladder::state::GlobalIntakeState;
use crate::motor_controllers::MotorController;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;

pub mod state;

#[cfg(test)]
mod tests;

enum IntakeActuatorAction {
    Rising,
    Falling,
    Stopped,
}

pub struct Intake {
    actuator: Box<dyn MotorController>,
    ladder: Box<dyn MotorController>,
    state: Arc<GlobalIntakeState>,
    life: Arc<GlobalLifeState>,
    enabled_cache: bool,
    action: IntakeActuatorAction,
}

impl Intake {
    pub fn new(ladder: Box<dyn MotorController>, actuator: Box<dyn MotorController>, state: Arc<GlobalIntakeState>, life: Arc<GlobalLifeState>) -> Self {
        let enabled_cache = state.get_enabled();
        Self {
            actuator,
            ladder,
            state,
            life,
            enabled_cache,
            action: IntakeActuatorAction::Stopped,
        }
    }

    pub fn enable(&mut self) {
        self.state.set_enabled(true);
        self.enabled_cache = true;
    }

    pub fn disable(&mut self) {
        self.state.set_enabled(false);
        self.enabled_cache = false;
        self.stop_digging();
        self.stop_actuators();
    }

    pub fn raise(&mut self) {
            self.actuator.set_speed(MH_ACTUATOR_RATE);
            self.action = IntakeActuatorAction::Rising;
    }

    pub fn lower(&mut self) {
            self.actuator.set_speed(-MH_ACTUATOR_RATE);
            self.action = IntakeActuatorAction::Falling;
    }

    pub fn stop_actuators(&mut self) {
        self.actuator.stop();
        self.action = IntakeActuatorAction::Stopped;
    }

    pub fn dig(&mut self) {
        if self.is_enabled() && self.life.is_alive() {
            self.ladder.set_speed(DIGGING_RATE);
        }
    }

    pub fn reverse(&mut self) {
        if self.is_enabled() && self.life.is_alive() {
            self.ladder.set_speed(-DIGGING_RATE);
        }
    }

    pub fn stop_digging(&mut self) {
        self.ladder.stop();
    }

    pub fn run_cycle(&mut self) {

    }

    #[inline]
    fn is_enabled(&self) -> bool {
        self.enabled_cache
    }
}