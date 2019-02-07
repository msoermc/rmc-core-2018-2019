use std::sync::Arc;

use crate::mechatronics::bucket_ladder::state::GlobalIntakeState;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::MotorController;

#[cfg(test)]
mod tests;

pub mod state;

pub struct Intake {
    actuators: Box<MotorController>,
    ladder: Box<MotorController>,
    state: Arc<GlobalIntakeState>,
    life: Arc<GlobalLifeState>,
}

impl Intake {
    pub fn new(ladder: Box<MotorController>, actuators: Box<MotorController>, state: Arc<GlobalIntakeState>, life: Arc<GlobalLifeState>) -> Self {
        Self {
            actuators,
            ladder,
            state,
            life,
        }
    }

    pub fn enable(&mut self) {
        self.state.set_enabled(true);
    }

    pub fn disable(&mut self) {
        self.state.set_enabled(false);
        self.stop_ladder();
        self.stop_actuators();
    }

    pub fn raise(&mut self) {
        if self.state.get_enabled() && self.life.is_alive() {
            self.actuators.set_speed(MH_ACTUATOR_RATE);
        } else {
            self.stop_actuators()
        }
    }

    pub fn lower(&mut self) {
        if self.state.get_enabled() && self.life.is_alive() {
            self.actuators.set_speed(-MH_ACTUATOR_RATE);
        } else {
            self.stop_actuators();
        }
    }

    pub fn stop_actuators(&mut self) {
        self.actuators.stop();
    }

    pub fn dig(&mut self) {
        if self.state.get_enabled() && self.life.is_alive() {
            self.ladder.set_speed(DIGGING_RATE);
        } else {
            self.stop_ladder();
        }
    }

    pub fn stop_ladder(&mut self) {
        self.ladder.stop();
    }

    pub fn run_cycle(&mut self) {
        if self.life.is_alive() && self.state.get_enabled() {
            // TODO
        } else {
            self.stop_ladder();
            self.stop_actuators();
        }
    }
}