use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::robot_map::*;
use crate::status::life::GlobalLifeStatus;
use std::sync::Arc;
use crate::mechatronics::material_handling::bucket_ladder::state::GlobalIntakeState;

#[cfg(test)]
mod tests;

pub mod state;

enum DiggerAction {

}

pub struct Ladder {
    is_enabled: bool,
    actuators: MotorGroup,
    digger: MotorGroup,
    state: GlobalIntakeState,
    life: GlobalLifeStatus,
}

impl Ladder {
    pub fn new(digger: MotorGroup, actuators: MotorGroup, life: GlobalLifeStatus) -> Self {
        unimplemented!()
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
        self.stop_digging();
        self.stop_actuators();
    }

    pub fn raise(&mut self) {
        //if self.is_enabled && self.life.is_alive() {
        //    self.actuators.set_speed(MH_ACTUATOR_RATE);
        //    self.actuator_state = ActuatorAction::RISING;
        //} else {
        //    self.stop_actuators()
        //}
    }

    pub fn lower(&mut self) {
        //if self.is_enabled && self.life.is_alive() {
        //    self.actuators.set_speed(-MH_ACTUATOR_RATE);
        //    self.actuator_state = ActuatorAction::LOWERING;
        //} else {
        //    self.stop_actuators();
        //}
    }

    pub fn stop_actuators(&mut self) {
        //self.actuators.stop();
        //self.actuator_state = ActuatorAction::STOPPED;
    }

    pub fn dig(&mut self) {
        //if self.is_enabled && self.life.is_alive() {
        //    self.digger.set_speed(DIGGING_RATE);
        //    self.digger_state = DiggerAction::DIGGING;
        //} else {
        //    self.stop_digging();
        //}
    }

    pub fn stop_digging(&mut self) {
        //self.digger.stop();
        //self.digger_state = DiggerAction::STOPPED;
    }

    pub fn run_cycle(&mut self) {
        //match self.actuator_state {
        //    ActuatorAction::RISING => self.raise(),
        //    ActuatorAction::LOWERING => self.lower(),
        //    ActuatorAction::STOPPED => self.stop_actuators(),
        //}
//
        //match self.digger_state {
        //    DiggerAction::DIGGING => self.dig(),
        //    DiggerAction::STOPPED => self.stop_digging(),
        //}
    }
}