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
    Digging(f32),
    Jammed,
}

pub struct BucketLadder {
    is_enabled: bool,
    actuators: MotorGroup,
    digger: MotorGroup,
    digger_state: DiggerState,
    actuator_state: ActuatorState,
    lower_limit_switch: Box<DigitalInput>,
    upper_limit_switch: Box<DigitalInput>,
}

impl BucketLadder {
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn raise(&mut self) {
        if let Ok(is_raised) = self.upper_limit_switch.get_value() {
            if is_raised {
                self.actuators.set_speed(MH_ACTUATOR_RATE);
                self.actuator_state = ActuatorState::Rising;
            }
        } else {
            error!("Failed to read limit switch value!");
            self.freeze_height();
        }
    }

    pub fn lower(&mut self) {
        if let Ok(is_lowered) = self.lower_limit_switch.get_value() {
            if is_lowered {
                self.actuators.set_speed(-MH_ACTUATOR_RATE);
                self.actuator_state = ActuatorState::Lowering;
            }
        } else {
            error!("Failed to read limit switch value!");
            self.freeze_height();
        }
    }

    pub fn freeze_height(&mut self) {
        self.actuators.stop();
        self.actuator_state = ActuatorState::Stopped;
    }

    pub fn dig(&mut self) {
        self.digger.set_speed(DIGGING_RATE);
    }

    pub fn stop_digging(&mut self) {
        self.digger.stop();
    }

    pub fn run_cycle(&mut self) {
        unimplemented!()
    }

    pub fn is_jammed(&self) -> bool {
        self.digger_state == DiggerState::Jammed
    }
}