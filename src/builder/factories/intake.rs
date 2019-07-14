use std::sync::Arc;

use crate::builder::factories::SubsystemFactory;
use crate::mechatronics::bucket_ladder::Intake;
use crate::motor_controllers::motor_group::MotorGroup;
use crate::motor_controllers::print_motor::PrintMotor;
use crate::motor_controllers::test_motor::TestMotor;
use crate::status::robot_state::GlobalRobotState;
use std::sync::mpsc::Sender;
use crate::arduino::{ArduinoMotor, ArduinoMessage};

pub struct ProductionIntakeFactory {
    state: Arc<GlobalRobotState>,
    io: Sender<ArduinoMessage>
}

pub struct TestIntakeFactory {
    state: Arc<GlobalRobotState>
}

pub struct PrintIntakeFactory {
    state: Arc<GlobalRobotState>
}

impl ProductionIntakeFactory {
    pub fn new(state: Arc<GlobalRobotState>, io: Sender<ArduinoMessage>) -> Self {
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
        let digger_motor = Box::new(ArduinoMotor::new(self.io.clone(), 5, state.get_intake().get_digger()));
        let actuator = Box::new(ArduinoMotor::new(self.io, 3, state.get_intake().get_actuator()));

        Intake::new(digger_motor, actuator, state.get_intake(), state.get_life())
    }
}

impl SubsystemFactory<Intake> for TestIntakeFactory {
    fn produce(self: Box<Self>) -> Intake {
        let state = &self.state;
        let digger_motor = Box::new(TestMotor::new(state.get_intake().get_digger()));
        let left_actuator = Box::new(TestMotor::new(state.get_intake().get_actuator()));

        let digger_group = Box::new(MotorGroup::new(vec![digger_motor], state.get_intake().get_digger()));

        Intake::new(digger_group, left_actuator, state.get_intake(), state.get_life())
    }
}

impl SubsystemFactory<Intake> for PrintIntakeFactory {
    fn produce(self: Box<Self>) -> Intake {
        let state = &self.state;
        let digger_motor = Box::new(PrintMotor::new("Digger", state.get_intake().get_digger()));
        let left_actuator = Box::new(PrintMotor::new("Actuators", state.get_intake().get_actuator()));

        let digger_group = Box::new(MotorGroup::new(vec![digger_motor], state.get_intake().get_digger()));

        Intake::new(digger_group, left_actuator, state.get_intake(), state.get_life())
    }
}