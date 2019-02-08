use std::sync::Arc;

use crate::mechatronics::bucket_ladder::state::GlobalIntakeState;
use crate::motor_controllers::MotorController;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;

pub mod state;

#[cfg(test)]
mod tests;

pub struct Intake {
    left_actuator: Box<MotorController>,
    right_actuator: Box<MotorController>,
    ladder: Box<MotorController>,
    state: Arc<GlobalIntakeState>,
    life: Arc<GlobalLifeState>,
}

impl Intake {
    pub fn new(ladder: Box<MotorController>, left_actuator: Box<MotorController>,
               right_actuator: Box<MotorController>, state: Arc<GlobalIntakeState>,
               life: Arc<GlobalLifeState>) -> Self {
        Self {
            left_actuator,
            right_actuator,
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
            self.left_actuator.set_speed(MH_ACTUATOR_RATE);
            self.right_actuator.set_speed(MH_ACTUATOR_RATE);
        } else {
            self.stop_actuators()
        }
    }

    pub fn lower(&mut self) {
        if self.state.get_enabled() && self.life.is_alive() {
            self.left_actuator.set_speed(-MH_ACTUATOR_RATE);
            self.right_actuator.set_speed(-MH_ACTUATOR_RATE);
        } else {
            self.stop_actuators();
        }
    }

    pub fn stop_actuators(&mut self) {
        self.left_actuator.stop();
        self.right_actuator.stop();
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