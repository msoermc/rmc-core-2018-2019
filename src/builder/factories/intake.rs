use std::rc::Rc;
use std::sync::Arc;

use crate::builder::factories::SubsystemFactory;
use crate::mechatronics::bucket_ladder::Intake;
use crate::motor_controllers::motor_group::MotorGroup;
use crate::motor_controllers::print_motor::PrintMotor;
use crate::motor_controllers::roboclaw::RoboClaw;
use crate::motor_controllers::test_motor::TestMotor;
use crate::pinouts::factories::IoFactory;
use crate::robot_map::ACTUATOR_PWM_CHIP;
use crate::robot_map::ACTUATOR_PWM_NUM;
use crate::robot_map::DIGGER_PWM_CHIP;
use crate::robot_map::DIGGER_PWM_NUM;
use crate::status::robot_state::GlobalRobotState;

pub struct ProductionIntakeFactory {
    state: Arc<GlobalRobotState>,
    io: Rc<IoFactory>,
}

pub struct TestIntakeFactory {
    state: Arc<GlobalRobotState>
}

pub struct PrintIntakeFactory {
    state: Arc<GlobalRobotState>
}

impl ProductionIntakeFactory {
    pub fn new(state: Arc<GlobalRobotState>, io: Rc<IoFactory>) -> Self {
        Self {
            state,
            io,
        }
    }
}

impl TestIntakeFactory {
    pub fn new(state: Arc<GlobalRobotState>) -> Self {
        Self {
            state
        }
    }
}

impl PrintIntakeFactory {
    pub fn new(state: Arc<GlobalRobotState>) -> Self {
        Self {
            state
        }
    }
}

impl ToString for ProductionIntakeFactory {
    fn to_string(&self) -> String {
        "production intake".to_owned()
    }
}

impl ToString for TestIntakeFactory {
    fn to_string(&self) -> String {
        "test intake".to_owned()
    }
}

impl ToString for PrintIntakeFactory {
    fn to_string(&self) -> String {
        "print intake".to_owned()
    }
}

impl SubsystemFactory<Intake> for ProductionIntakeFactory {
    fn produce(self: Box<Self>) -> Intake {
        let state = &self.state;
        let digger_pwm = self.io.generate_pwm(DIGGER_PWM_CHIP, DIGGER_PWM_NUM);
        let left_pwm = self.io.generate_pwm(ACTUATOR_PWM_CHIP, ACTUATOR_PWM_NUM);
        let digger_motor = Box::new(RoboClaw::new(digger_pwm));
        let left_actuator = Box::new(RoboClaw::new(left_pwm));

        let actuator_group = Box::new(MotorGroup::new(vec![left_actuator], state.get_intake().get_actuator()));
        let digger_group = Box::new(MotorGroup::new(vec![digger_motor], state.get_intake().get_ladder()));

        Intake::new(digger_group, actuator_group, state.get_intake(), state.get_life())
    }
}

impl SubsystemFactory<Intake> for TestIntakeFactory {
    fn produce(self: Box<Self>) -> Intake {
        let state = &self.state;
        let digger_motor = Box::new(TestMotor::new(state.get_intake().get_ladder()));
        let left_actuator = Box::new(TestMotor::new(state.get_intake().get_actuator()));

        let digger_group = Box::new(MotorGroup::new(vec![digger_motor], state.get_intake().get_ladder()));

        Intake::new(digger_group, left_actuator, state.get_intake(), state.get_life())
    }
}

impl SubsystemFactory<Intake> for PrintIntakeFactory {
    fn produce(self: Box<Self>) -> Intake {
        let state = &self.state;
        let digger_motor = Box::new(PrintMotor::new("Digger", state.get_intake().get_ladder()));
        let left_actuator = Box::new(PrintMotor::new("Actuators", state.get_intake().get_actuator()));

        let digger_group = Box::new(MotorGroup::new(vec![digger_motor], state.get_intake().get_ladder()));

        Intake::new(digger_group, left_actuator, state.get_intake(), state.get_life())
    }
}