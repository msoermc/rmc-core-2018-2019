use std::rc::Rc;
use std::sync::Arc;

use crate::builder::factories::SubsystemFactory;
use crate::mechatronics::dumper::Dumper;
use crate::motor_controllers::motor_group::MotorGroup;
use crate::motor_controllers::print_motor::PrintMotor;
use crate::motor_controllers::test_motor::TestMotor;
use crate::pinouts::factories::IoFactory;
use crate::status::robot_state::GlobalRobotState;

pub struct ProductionDumperFactory {
    state: Arc<GlobalRobotState>,
    io: Rc<IoFactory>,
}

pub struct TestDumperFactory {
    state: Arc<GlobalRobotState>
}

pub struct PrintDumperFactory {
    state: Arc<GlobalRobotState>
}

impl ProductionDumperFactory {
    pub fn new(state: Arc<GlobalRobotState>, io: Rc<IoFactory>) -> Self {
        Self {
            state,
            io,
        }
    }
}

impl TestDumperFactory {
    pub fn new(state: Arc<GlobalRobotState>) -> Self {
        Self {
            state
        }
    }
}

impl PrintDumperFactory {
    pub fn new(state: Arc<GlobalRobotState>) -> Self {
        Self {
            state
        }
    }
}

impl ToString for ProductionDumperFactory {
    fn to_string(&self) -> String {
        "production dumper".to_owned()
    }
}

impl ToString for TestDumperFactory {
    fn to_string(&self) -> String {
        "test dumper".to_owned()
    }
}

impl ToString for PrintDumperFactory {
    fn to_string(&self) -> String {
        "print dumper".to_owned()
    }
}

impl SubsystemFactory<Dumper> for ProductionDumperFactory {
    fn produce(&self) -> Dumper {
        unimplemented!()
    }
}

impl SubsystemFactory<Dumper> for TestDumperFactory {
    fn produce(&self) -> Dumper {
        let state = &self.state;
        let dumper_motor = Box::new(TestMotor::new(state.get_dumper().get_motor()));

        let dumper_group = Box::new(MotorGroup::new(vec![dumper_motor], state.get_dumper().get_motor()));

        Dumper::new(state.get_life(), dumper_group, state.get_dumper())
    }
}

impl SubsystemFactory<Dumper> for PrintDumperFactory {
    fn produce(&self) -> Dumper {
        let state = &self.state;
        let dumper_motor = Box::new(PrintMotor::new("Dumper", state.get_dumper().get_motor()));

        let dumper_group = Box::new(MotorGroup::new(vec![dumper_motor], state.get_dumper().get_motor()));

        Dumper::new(state.get_life(), dumper_group, state.get_dumper())
    }
}