use crate::builder::factories::SubsystemFactory;
use crate::mechatronics::bucket_ladder::Intake;
use crate::status::robot_state::GlobalRobotState;
use std::sync::Arc;
use crate::pinouts::factories::IoFactory;
use std::rc::Rc;
use crate::motor_controllers::test_motor::TestMotor;
use crate::motor_controllers::motor_group::MotorGroup;

pub struct ProductionIntakeFactory {
    state: Arc<GlobalRobotState>,
    io: Rc<IoFactory>
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
            io
        }
    }
}

impl TestIntakeFactory {
    pub fn new(state: Arc<GlobalRobotState>) -> Self {
        Self {
            state
        }
    }}

impl PrintIntakeFactory {
    pub fn new(state: Arc<GlobalRobotState>) -> Self {
        Self {
            state
        }
    }}

impl SubsystemFactory<Intake> for ProductionIntakeFactory {
    fn produce(&self) -> Intake {
        unimplemented!()
    }
}

impl SubsystemFactory<Intake> for TestIntakeFactory {
    fn produce(&self) -> Intake {
        let state = &self.state;
        let digger_motor = Box::new(TestMotor::new(state.get_intake().get_ladder().get_motor()));
        let left_actuator = Box::new(TestMotor::new(state.get_intake().get_left_actuator().get_motor()));
        let right_actuator = Box::new(TestMotor::new(state.get_intake().get_right_actuator().get_motor()));

        let digger_group = Box::new(MotorGroup::new(vec![digger_motor], state.get_intake().get_ladder().get_motor()));

        Intake::new(digger_group, left_actuator, right_actuator, state.get_intake(), state.get_life())
    }
}

impl SubsystemFactory<Intake> for PrintIntakeFactory {
    fn produce(&self) -> Intake {
        unimplemented!()
    }
}