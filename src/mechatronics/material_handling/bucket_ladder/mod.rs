use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::mechatronics::GlobalLifeStatus;
use crate::robot_map::*;

#[cfg(test)]
mod tests;

enum ActuatorState {
    Rising,
    Lowering,
    Stopped,
}

enum DiggerState {
    Digging,
    Stopped,
}

pub struct BucketLadder {
    is_enabled: bool,
    actuators: MotorGroup,
    digger: MotorGroup,
    digger_state: DiggerState,
    actuator_state: ActuatorState,
    life: GlobalLifeStatus,
}

impl BucketLadder {
    pub fn new(digger: MotorGroup, actuators: MotorGroup, life: GlobalLifeStatus) -> Self {
        Self {
            is_enabled: true,
            actuators,
            digger,
            digger_state: DiggerState::Stopped,
            actuator_state: ActuatorState::Stopped,
            life,
        }
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
        if self.is_enabled && self.life.is_alive() {
            self.actuators.set_speed(MH_ACTUATOR_RATE);
            self.actuator_state = ActuatorState::Rising;
        } else {
            self.stop_actuators()
        }
    }

    pub fn lower(&mut self) {
        if self.is_enabled && self.life.is_alive() {
            self.actuators.set_speed(-MH_ACTUATOR_RATE);
            self.actuator_state = ActuatorState::Lowering;
        } else {
            self.stop_actuators();
        }
    }

    pub fn stop_actuators(&mut self) {
        self.actuators.stop();
        self.actuator_state = ActuatorState::Stopped;
    }

    pub fn dig(&mut self) {
        if self.is_enabled && self.life.is_alive() {
            self.digger.set_speed(DIGGING_RATE);
            self.digger_state = DiggerState::Digging;
        } else {
            self.stop_digging();
        }
    }

    pub fn stop_digging(&mut self) {
        self.digger.stop();
        self.digger_state = DiggerState::Stopped;
    }

    pub fn run_cycle(&mut self) {
        match self.actuator_state {
            ActuatorState::Rising => self.raise(),
            ActuatorState::Lowering => self.lower(),
            ActuatorState::Stopped => self.stop_actuators(),
        }

        match self.digger_state {
            DiggerState::Digging => self.dig(),
            DiggerState::Stopped => self.stop_digging(),
        }
    }
}