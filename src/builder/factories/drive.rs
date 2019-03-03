use std::rc::Rc;
use std::sync::Arc;

use crate::builder::factories::SubsystemFactory;
use crate::mechatronics::drive_train::DriveTrain;
use crate::motor_controllers::hover_board::HoverBoardMotor;
use crate::motor_controllers::motor_group::MotorGroup;
use crate::motor_controllers::print_motor::PrintMotor;
use crate::motor_controllers::test_motor::TestMotor;
use crate::pinouts::factories::IoFactory;
use crate::robot_map::*;
use crate::status::robot_state::GlobalRobotState;

pub struct ProductionDriveFactory {
    state: Arc<GlobalRobotState>,
    io: Rc<IoFactory>,
}

pub struct TestDriveFactory {
    state: Arc<GlobalRobotState>
}

pub struct PrintDriveFactory {
    state: Arc<GlobalRobotState>
}

impl ProductionDriveFactory {
    pub fn new(state: Arc<GlobalRobotState>, io: Rc<IoFactory>) -> Self {
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
    fn produce(&self) -> DriveTrain {
        let io_factory = &self.io;

        let left_front_pwm = io_factory.generate_analog_output(FRONT_LEFT_PWM_CHIP, FRONT_LEFT_PWM_NUMBER);
        let right_front_pwm = io_factory.generate_analog_output(FRONT_RIGHT_PWM_CHIP, FRONT_RIGHT_PWM_NUMBER);
        let left_rear_pwm = io_factory.generate_analog_output(REAR_LEFT_PWM_CHIP, REAR_LEFT_PWM_NUMBER);
        let right_rear_pwm = io_factory.generate_analog_output(REAR_RIGHT_PWM_CHIP, REAR_RIGHT_PWM_NUMBER);

        let front_right_direction = io_factory.generate_digital_output(FRONT_RIGHT_DIRECTION);
        let front_left_direction = io_factory.generate_digital_output(FRONT_LEFT_DIRECTION);
        let rear_right_direction = io_factory.generate_digital_output(REAR_RIGHT_DIRECTION);
        let rear_left_direction = io_factory.generate_digital_output(REAR_LEFT_DIRECTION);

        let front_right_motor = Box::new(HoverBoardMotor::new(right_front_pwm, front_right_direction));
        let front_left_motor = Box::new(HoverBoardMotor::new(left_front_pwm, front_left_direction));
        let rear_right_motor = Box::new(HoverBoardMotor::new(right_rear_pwm, rear_right_direction));
        let rear_left_motor = Box::new(HoverBoardMotor::new(left_rear_pwm, rear_left_direction));

        let left_drive = Box::new(MotorGroup::new(vec![front_left_motor, rear_left_motor], self.state.get_drive().get_left()));
        let right_drive = Box::new(MotorGroup::new(vec![front_right_motor, rear_right_motor], self.state.get_drive().get_right()));

        DriveTrain::new(self.state.get_drive(), left_drive, right_drive, self.state.get_life())
    }
}

impl SubsystemFactory<DriveTrain> for TestDriveFactory {
    fn produce(&self) -> DriveTrain {
        let state = &self.state;
        let left_motor = Box::new(TestMotor::new(state.get_drive().get_left()));
        let right_motor = Box::new(TestMotor::new(state.get_drive().get_right()));

        let left_group = Box::new(MotorGroup::new(vec![left_motor], state.get_drive().get_left()));
        let right_group = Box::new(MotorGroup::new(vec![right_motor], state.get_drive().get_right()));

        DriveTrain::new(self.state.get_drive(), left_group, right_group, self.state.get_life())
    }
}

impl SubsystemFactory<DriveTrain> for PrintDriveFactory {
    fn produce(&self) -> DriveTrain {
        let state = &self.state;
        let left_motor = Box::new(PrintMotor::new("Left", state.get_drive().get_left()));
        let right_motor = Box::new(PrintMotor::new("Right", state.get_drive().get_right()));

        let left_group = Box::new(MotorGroup::new(vec![left_motor], state.get_drive().get_left()));
        let right_group = Box::new(MotorGroup::new(vec![right_motor], state.get_drive().get_right()));

        DriveTrain::new(self.state.get_drive(), left_group, right_group, self.state.get_life())
    }
}