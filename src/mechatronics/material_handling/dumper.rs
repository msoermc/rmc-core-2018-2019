use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::robot_map::*;

pub struct Dumper {
    is_enabled: bool,
    motors: MotorGroup,
    state: DumperState,
}

enum DumperState {
    Dumping,
    Resetting,
    Stopped,
}

impl Dumper {
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn dump(&mut self) {
        self.motors.set_speed(DUMPING_RATE);
        self.state = DumperState::Dumping;
    }

    pub fn reset(&mut self) {
        self.motors.set_speed(DUMPER_RESET_RATE);
        self.state = DumperState::Resetting;
    }

    pub fn stop(&mut self) {
        self.motors.set_speed(0.0);
        self.state = DumperState::Stopped;
    }

    pub fn run_cycle(&mut self) {
        match self.state {
            DumperState::Dumping => self.dump(),
            DumperState::Resetting => self.reset(),
            DumperState::Stopped => self.stop(),
        }
    }
}