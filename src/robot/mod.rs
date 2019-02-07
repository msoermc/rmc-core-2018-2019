use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread::spawn;

use rocket::local::Client;
use rocket::Rocket;

use crate::comms;
use crate::devices::enable_pins;
use crate::devices::motor_controllers::hover_board::HoverBoardMotor;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::print_motor::PrintMotor;
use crate::devices::sysfs_pin_wrappers::SysfsPin;
use crate::devices::sysfs_pwm_wrappers::SysfsPwm;
use crate::framework::Runnable;
use crate::mechatronics::bucket_ladder::Intake;
use crate::mechatronics::controller::RobotController;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::dumper::Dumper;
use crate::mechatronics::MechatronicsMessageSender;
use crate::robot_map::*;
use crate::status::life::GlobalLifeState;
use crate::status::robot_state::GlobalRobotState;

pub struct RobotBuilder {
    left_drive: Box<MotorController>,
    right_drive: Box<MotorController>,
    digger: Box<MotorController>,
    left_actuator: Box<MotorController>,
    right_actuator: Box<MotorController>,
    dumper: Box<MotorController>,
    state: GlobalRobotState,
}

impl RobotBuilder {
    pub fn use_custom_drive(&mut self, left: Box<MotorController>, right: Box<MotorController>) {
        self.left_drive = left;
        self.right_drive = right;
    }

    pub fn use_custom_intake(&mut self, digger: Box<MotorController>,
                             left_actuator: Box<MotorController>,
                             right_actuator: Box<MotorController>) {
        self.digger = digger;
        self.left_actuator = left_actuator;
        self.right_actuator = right_actuator
    }

    pub fn use_custom_dumper(&mut self, dumper: Box<MotorController>) {
        self.dumper = dumper;
    }

    pub fn use_real_drive(&mut self) {
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

    pub fn new() -> Self {
        let state = GlobalRobotState::new();
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
        }
    }

    pub fn build(self) -> Robot {
        let (controller_sender, controller_receiver) = channel();

        let robot_state = Arc::new(GlobalRobotState::new());

        let robot_view = MechatronicsMessageSender::new(controller_sender, robot_state.clone());
        let bfr = comms::stage(robot_view, robot_state.clone());

        let drive_train = DriveTrain::new(robot_state.get_drive(), self.left_drive, self.right_drive, robot_state.get_life());

        let digger = Intake::new(self.digger, self.left_actuator, robot_state.get_intake(), robot_state.get_life());

        let dumper = Dumper::new(robot_state.get_life().clone(), self.dumper, robot_state.get_dumper());

        let robot_controller = RobotController::new(controller_receiver, drive_train, dumper, digger, robot_state.get_life());

        Robot::new(robot_controller, bfr)
    }
}

pub struct Robot {
    controller: RobotController,
    bfr: Rocket,
}

impl Robot {
    fn new(controller: RobotController, bfr: Rocket) -> Self {
        Self {
            controller,
            bfr,
        }
    }

    pub fn launch(self) {
        let bfr = self.bfr;
        let mut controller = self.controller;
        let controller_thread = spawn(move || controller.start());
        let _rocket_thread = spawn(move || bfr.launch());

        controller_thread.join().expect("Controller thread panicked!");
    }

    pub fn launch_tester(self) -> Client {
        let bfr = self.bfr;
        let mut controller = self.controller;

        spawn(move || controller.start());
        Client::new(bfr).expect("Failed to launch client!")
    }
}