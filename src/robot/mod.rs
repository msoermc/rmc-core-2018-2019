use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread::spawn;

use rocket::local::Client;
use rocket::Rocket;

use crate::benchmarking::ControllerBench;
use crate::comms;
use crate::framework::Runnable;
use crate::mechatronics::bucket_ladder::Intake;
use crate::mechatronics::controller::RobotController;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::dumper::Dumper;
use crate::mechatronics::RobotMessenger;
use crate::motor_controllers::hover_board::HoverBoardMotor;
use crate::motor_controllers::motor_group::MotorGroup;
use crate::motor_controllers::MotorController;
use crate::motor_controllers::print_motor::PrintMotor;
use crate::motor_controllers::test_motor::TestMotor;
use crate::pinouts::enable_pins;
use crate::pinouts::sysfs_pin_wrappers::SysfsPin;
use crate::pinouts::sysfs_pwm_wrappers::SysfsPwm;
use crate::robot_map::*;
use crate::status::robot_state::GlobalRobotState;
use crate::mechatronics::commands::RobotCommandFactory;
use std::sync::mpsc::sync_channel;

/// Assembles the robot from components using the builder design pattern.
/// If no preparation instructions are given, a default configuration using `PrintMotors` is assumed.
pub struct RobotBuilder {
    left_drive: Box<MotorController>,
    right_drive: Box<MotorController>,
    digger: Box<MotorController>,
    left_actuator: Box<MotorController>,
    right_actuator: Box<MotorController>,
    dumper: Box<MotorController>,
    state: Arc<GlobalRobotState>,
    bench: Option<ControllerBench>,
}

impl RobotBuilder {
    /// Returns the state object which will be used by the constructed robot object.
    pub fn get_state(&self) -> Arc<GlobalRobotState> {
        self.state.clone()
    }

    /// Instructs the builder to prepare the robot to use a test setup.
    pub fn with_test(&mut self) {
        let state = &self.state;
        let left_motor = Box::new(TestMotor::new(state.get_drive().get_left()));
        let right_motor = Box::new(TestMotor::new(state.get_drive().get_right()));
        let digger_motor = Box::new(TestMotor::new(state.get_intake().get_ladder().get_motor()));
        let left_actuator = Box::new(TestMotor::new(state.get_intake().get_left_actuator().get_motor()));
        let right_actuator = Box::new(TestMotor::new(state.get_intake().get_right_actuator().get_motor()));
        let dumper_motor = Box::new(TestMotor::new(state.get_dumper().get_motor()));

        let left_group = Box::new(MotorGroup::new(vec![left_motor], state.get_drive().get_left()));
        let right_group = Box::new(MotorGroup::new(vec![right_motor], state.get_drive().get_right()));
        let digger_group = Box::new(MotorGroup::new(vec![digger_motor], state.get_intake().get_ladder().get_motor()));
        let dumper_group = Box::new(MotorGroup::new(vec![dumper_motor], state.get_dumper().get_motor()));

        self.left_actuator = left_actuator;
        self.right_actuator = right_actuator;
        self.left_drive = left_group;
        self.right_drive = right_group;
        self.digger = digger_group;
        self.dumper = dumper_group;
    }

    pub fn with_bench(&mut self) {
        let bench = ControllerBench::new(self.state.get_cycle_counter(), self.state.get_cycles_per_second());
        self.bench = Some(bench);
    }

    /// Instructs the builder to prepare the robot to use the real setup for the robot, with the appropriate
    /// pinouts and hardware configuration.
    pub fn with_real(&mut self) {
        enable_pins().expect("Failed to enable pins!");

        let left_front_pwm = Box::new(SysfsPwm::create(FRONT_LEFT_PWM_CHIP, FRONT_LEFT_PWM_NUMBER, FRONT_LEFT_DRIVE_STRING).expect("Front left pwm"));
        let right_front_pwm = Box::new(SysfsPwm::create(FRONT_RIGHT_PWM_CHIP, FRONT_RIGHT_PWM_NUMBER, FRONT_RIGHT_DRIVE_STRING).expect("Front right pwm"));
        let left_rear_pwm = Box::new(SysfsPwm::create(REAR_LEFT_PWM_CHIP, REAR_LEFT_PWM_NUMBER, REAR_LEFT_DRIVE_STRING).expect("Rear left pwm"));
        let right_rear_pwm = Box::new(SysfsPwm::create(REAR_RIGHT_PWM_CHIP, REAR_RIGHT_PWM_NUMBER, REAR_RIGHT_DRIVE_STRING).expect("Rear right pwm"));

        let front_right_direction = Box::new(SysfsPin::create(FRONT_RIGHT_DIRECTION, FRONT_RIGHT_DIRECTION_STRING).expect("Front right direction"));
        let front_left_direction = Box::new(SysfsPin::create(FRONT_LEFT_DIRECTION, FRONT_LEFT_DIRECTION_STRING).expect("Front left direction"));
        let rear_right_direction = Box::new(SysfsPin::create(REAR_RIGHT_DIRECTION, REAR_RIGHT_DIRECTION_STRING).expect("Rear right direction"));
        let rear_left_direction = Box::new(SysfsPin::create(REAR_LEFT_DIRECTION, REAR_LEFT_DIRECTION_STRING).expect("Rear left direction"));

        let front_right_motor = Box::new(HoverBoardMotor::new(right_front_pwm, front_right_direction));
        let front_left_motor = Box::new(HoverBoardMotor::new(left_front_pwm, front_left_direction));
        let rear_right_motor = Box::new(HoverBoardMotor::new(right_rear_pwm, rear_right_direction));
        let rear_left_motor = Box::new(HoverBoardMotor::new(left_rear_pwm, rear_left_direction));

        self.left_drive = Box::new(MotorGroup::new(vec![front_left_motor, rear_left_motor], self.state.get_drive().get_left()));
        self.right_drive = Box::new(MotorGroup::new(vec![front_right_motor, rear_right_motor], self.state.get_drive().get_right()));
    }

    /// Constructs the robot, using a default configuration with `PrintMotor`s.
    pub fn new() -> Self {
        let state = Arc::new(GlobalRobotState::new());
        let left_motor = Box::new(PrintMotor::new("Left", state.get_drive().get_left()));
        let right_motor = Box::new(PrintMotor::new("Right", state.get_drive().get_right()));
        let digger_motor = Box::new(PrintMotor::new("Digger", state.get_intake().get_ladder().get_motor()));
        let left_actuator = Box::new(PrintMotor::new("LA", state.get_intake().get_left_actuator().get_motor()));
        let right_actuator = Box::new(PrintMotor::new("RA", state.get_intake().get_right_actuator().get_motor()));
        let dumper_motor = Box::new(PrintMotor::new("Dumper", state.get_dumper().get_motor()));

        let left_group = Box::new(MotorGroup::new(vec![left_motor], state.get_drive().get_left()));
        let right_group = Box::new(MotorGroup::new(vec![right_motor], state.get_drive().get_right()));
        let digger_group = Box::new(MotorGroup::new(vec![digger_motor], state.get_intake().get_ladder().get_motor()));
        let dumper_group = Box::new(MotorGroup::new(vec![dumper_motor], state.get_dumper().get_motor()));


        Self {
            left_drive: left_group,
            right_drive: right_group,
            digger: digger_group,
            left_actuator,
            right_actuator,
            dumper: dumper_group,
            state,
            bench: None,
        }
    }

    /// Builds the robot from the configured preparations.
    pub fn build(self) -> Robot {
        let (controller_sender, controller_receiver) = sync_channel(20);

        let command_factory = RobotCommandFactory::new();

        let robot_view = RobotMessenger::new(controller_sender);
        let bfr = comms::stage(robot_view, self.state.clone(), command_factory);

        let drive_train = DriveTrain::new(self.state.get_drive(), self.left_drive, self.right_drive, self.state.get_life());

        let digger = Intake::new(self.digger, self.left_actuator, self.right_actuator, self.state.get_intake(), self.state.get_life());

        let dumper = Dumper::new(self.state.get_life().clone(), self.dumper, self.state.get_dumper());

        let robot_controller = RobotController::new(controller_receiver, drive_train, dumper, digger, self.state.get_life(), self.state.get_cycle_counter());

        Robot::new(robot_controller, bfr, self.bench)
    }
}

/// A built and configured robot which can be launched in either a test mode, which returns back
/// a rocket `Client` object, or normally.
pub struct Robot {
    controller: RobotController,
    bfr: Rocket,
    bench: Option<ControllerBench>,
}

impl Robot {
    fn new(controller: RobotController, bfr: Rocket, bench: Option<ControllerBench>) -> Self {
        Self {
            controller,
            bfr,
            bench,
        }
    }

    /// Launches the robot, taking over the current thread.
    /// This method consumes the robot.
    pub fn launch(self) {
        let bfr = self.bfr;
        let mut controller = self.controller;
        let controller_thread = spawn(move || controller.start());
        let _rocket_thread = spawn(move || bfr.launch());
        self.bench.map(|bench| spawn(move || {
            bench.launch();
        }));

        controller_thread.join().expect("Controller thread panicked!");
    }

    /// Launches the robot in test mode in a separate thread.
    /// This method consumes the robot and returns a `Client` object which can be used for sending requests
    /// to the robot.
    pub fn launch_tester(self) -> Client {
        let bfr = self.bfr;
        let mut controller = self.controller;
        spawn(move || controller.start());
        self.bench.map(|bench| spawn(move || {
            bench.launch();
        }));
        Client::new(bfr).expect("Failed to launch client!")
    }
}