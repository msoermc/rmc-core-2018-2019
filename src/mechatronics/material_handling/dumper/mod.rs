use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::robot_map::*;
use crate::mechatronics::GlobalLifeStatus;

#[cfg(test)]
mod tests;

pub struct Dumper {
    is_enabled: bool,
    motors: MotorGroup,
    state: DumperState,
    life: GlobalLifeStatus,
}

enum DumperState {
    Dumping,
    Resetting,
    Stopped,
}

impl Dumper {
    pub fn new(life: GlobalLifeStatus, motors: MotorGroup) -> Self {
        Self {
            is_enabled: true,
            motors,
            state: DumperState::Stopped,
            life,
        }
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
        self.stop();
    }

    pub fn dump(&mut self) {
        if self.is_enabled && self.life.is_alive() {
            self.motors.set_speed(DUMPING_RATE);
            self.state = DumperState::Dumping;
        }
    }

    pub fn reset(&mut self) {
        if self.is_enabled && self.life.is_alive() {
            self.motors.set_speed(DUMPER_RESET_RATE);
            self.state = DumperState::Resetting;
        }
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