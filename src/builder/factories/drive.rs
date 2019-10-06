use std::sync::Arc;

use crate::builder::factories::SubsystemFactory;
use crate::mechatronics::drive_train::DriveTrain;
use crate::motor_controllers::motor_group::MotorGroup;
use crate::motor_controllers::print_motor::PrintMotor;
use crate::motor_controllers::test_motor::TestMotor;
use crate::status::robot_state::GlobalRobotState;
use crate::motor_controllers::decorators::inversion::InvertedMotor;
use crate::arduino::{ArduinoMotor, ArduinoMessage};
use std::sync::mpsc::Sender;

pub struct ProductionDriveFactory {
    state: Arc<GlobalRobotState>,
    io: Sender<ArduinoMessage>,
}

pub struct TestDriveFactory {
    state: Arc<GlobalRobotState>
}

pub struct PrintDriveFactory {
    state: Arc<GlobalRobotState>
}

impl ProductionDriveFactory {
    pub fn new(state: Arc<GlobalRobotState>, io: Sender<ArduinoMessage>) -> Self {
        Self {
            state,
            io,
        }
    }
}

impl TestDriveFactory {
    pub fn new(state: Arc<GlobalRobotState>) -> Self {
        Self {
            state
        }
    }
}

impl PrintDriveFactory {
    pub fn new(state: Arc<GlobalRobotState>) -> Self {
        Self {
            state
        }
    }
}

impl ToString for ProductionDriveFactory {
    fn to_string(&self) -> String {
        "production drive".to_owned()
    }
}

impl ToString for TestDriveFactory {
    fn to_string(&self) -> String {
        "test drive".to_owned()
    }
}

impl ToString for PrintDriveFactory {
    fn to_string(&self) -> String {
        "print drive".to_owned()
    }
}

impl SubsystemFactory<DriveTrain> for ProductionDriveFactory {
    fn produce(self: Box<Self>) -> DriveTrain {
        let left_drive = Box::new(ArduinoMotor::new(self.io.clone(), 1, self.state.get_drive().get_left()));
        let right_drive = Box::new(ArduinoMotor::new(self.io, 2, self.state.get_drive().get_right()));

//        let left_drive = Box::new(InvertedMotor::new(left_drive));

        DriveTrain::new(self.state.get_drive(), left_drive, right_drive, self.state.get_life())
    }
}

impl SubsystemFactory<DriveTrain> for TestDriveFactory {
    fn produce(self: Box<Self>) -> DriveTrain {
        let state = &self.state;
        let left_motor = Box::new(TestMotor::new(state.get_drive().get_left()));
        let right_motor = Box::new(TestMotor::new(state.get_drive().get_right()));

        DriveTrain::new(self.state.get_drive(), left_motor, right_motor, self.state.get_life())
    }
}

impl SubsystemFactory<DriveTrain> for PrintDriveFactory {
    fn produce(self: Box<Self>) -> DriveTrain {
        let state = &self.state;
        let left_motor = Box::new(PrintMotor::new("Left", state.get_drive().get_left()));
        let right_motor = Box::new(PrintMotor::new("Right", state.get_drive().get_right()));

        let left_group = Box::new(MotorGroup::new(vec![left_motor], state.get_drive().get_left()));
        let right_group = Box::new(MotorGroup::new(vec![right_motor], state.get_drive().get_right()));

        DriveTrain::new(self.state.get_drive(), left_group, right_group, self.state.get_life())
    }
}