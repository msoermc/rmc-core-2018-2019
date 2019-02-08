use std::sync::Arc;

use crate::mechatronics::bucket_ladder::state::GlobalIntakeState;
use crate::motor_controllers::MotorController;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;

pub mod state;

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::motor_controllers::test_motor::TestMotor;

    use super::*;

    struct IntakeEnvironment {
        pub state: Arc<GlobalIntakeState>,
        pub life: Arc<GlobalLifeState>,
        pub intake: Intake,
    }

    fn create_environment() -> IntakeEnvironment {
        let state = Arc::new(GlobalIntakeState::new());
        let left = Box::new(TestMotor::new(state.get_left_actuator().get_motor()));
        let right = Box::new(TestMotor::new(state.get_right_actuator().get_motor()));
        let ladder = Box::new(TestMotor::new(state.get_ladder().get_motor()));
        let life = Arc::new(GlobalLifeState::new());
        let intake = Intake::new(ladder, left, right, state.clone(), life.clone());

        IntakeEnvironment {
            state,
            life,
            intake,
        }
    }

    #[test]
    fn test_setup() {
        let environment = create_environment();

        assert_eq!(0.0, environment.state.get_ladder().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_ladder().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_motor().get_speed());
        assert_eq!(false, environment.state.get_enabled());
        assert_eq!(false, environment.state.get_current_state().get_enabled());
    }

    #[test]
    fn test_raise_actuators() {
        let mut environment = create_environment();
        environment.intake.enable();

        environment.intake.raise();
        assert_eq!(MH_ACTUATOR_RATE, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(MH_ACTUATOR_RATE, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(MH_ACTUATOR_RATE, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(MH_ACTUATOR_RATE, environment.state.get_right_actuator().get_motor().get_speed());

        environment.intake.run_cycle();
        assert_eq!(MH_ACTUATOR_RATE, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(MH_ACTUATOR_RATE, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(MH_ACTUATOR_RATE, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(MH_ACTUATOR_RATE, environment.state.get_right_actuator().get_motor().get_speed());

        environment.intake.disable();
        environment.intake.raise();
        assert_eq!(0.0, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_motor().get_speed());
        environment.intake.run_cycle();
        assert_eq!(0.0, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_motor().get_speed());

        environment.intake.enable();
        environment.intake.raise();
        environment.life.kill();
        environment.intake.run_cycle();
        assert_eq!(0.0, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_motor().get_speed());
    }

    #[test]
    fn test_lower_actuators() {
        let mut environment = create_environment();
        environment.intake.enable();

        environment.intake.lower();
        assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_right_actuator().get_motor().get_speed());

        environment.intake.run_cycle();
        assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(-MH_ACTUATOR_RATE, environment.state.get_right_actuator().get_motor().get_speed());

        environment.intake.disable();
        environment.intake.lower();
        assert_eq!(0.0, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_motor().get_speed());
        environment.intake.run_cycle();
        assert_eq!(0.0, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_motor().get_speed());

        environment.intake.enable();
        environment.intake.lower();
        environment.life.kill();
        environment.intake.run_cycle();
        assert_eq!(0.0, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_motor().get_speed());
    }

    #[test]
    fn test_stop_actuators() {
        let mut environment = create_environment();
        environment.intake.enable();
        environment.intake.raise();
        environment.intake.stop_actuators();
        assert_eq!(0.0, environment.state.get_left_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_left_actuator().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_current_state().get_motor().get_speed());
        assert_eq!(0.0, environment.state.get_right_actuator().get_motor().get_speed());
    }

    #[test]
    fn test_enabling() {
        let mut environment = create_environment();
        environment.intake.enable();
        assert_eq!(true, environment.state.get_enabled());
        assert_eq!(true, environment.state.get_current_state().get_enabled());
        environment.intake.disable();
        assert_eq!(false, environment.state.get_enabled());
        assert_eq!(false, environment.state.get_current_state().get_enabled());
    }
}