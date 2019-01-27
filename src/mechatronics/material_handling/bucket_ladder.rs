use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::sensors::DigitalInput;
use crate::robot_map::*;

#[derive(Clone, PartialEq)]
enum ActuatorState {
    Rising,
    Lowering,
    Stopped,
}

#[derive(Clone, PartialEq)]
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
}

impl BucketLadder {
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn raise(&mut self) {
        self.actuators.set_speed(MH_ACTUATOR_RATE);
        self.actuator_state = ActuatorState::Rising;
    }

    pub fn lower(&mut self) {
        self.actuators.set_speed(-MH_ACTUATOR_RATE);
        self.actuator_state = ActuatorState::Lowering;
    }

    pub fn stop_actuators(&mut self) {
        self.actuators.stop();
        self.actuator_state = ActuatorState::Stopped;
    }

    pub fn dig(&mut self) {
        self.digger.set_speed(DIGGING_RATE);
        self.digger_state = DiggerState::Digging;
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