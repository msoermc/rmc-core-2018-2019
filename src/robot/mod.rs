use std::sync::mpsc::channel;
use std::thread::spawn;

use rocket::local::Client;
use rocket::Rocket;

use crate::comms;
use crate::devices::enable_pins;
use crate::devices::motor_controllers::hover_board::HoverBoardMotor;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::print_motor::PrintMotor;
use crate::devices::sysfs_pin_wrappers::SysfsPin;
use crate::devices::sysfs_pwm_wrappers::SysfsPwm;
use crate::framework::Runnable;
use crate::mechatronics::controller::RobotController;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::material_handling::bucket_ladder::BucketLadder;
use crate::mechatronics::material_handling::dumper::Dumper;
use crate::mechatronics::MechatronicsMessageSender;
use crate::robot_map::*;
use crate::status::life::GlobalLifeStatus;

pub struct RobotBuilder {
    left_drive: MotorGroup,
    right_drive: MotorGroup,
    digger: MotorGroup,
    rails: MotorGroup,
    dumper: MotorGroup,
}

impl RobotBuilder {
    pub fn use_custom_drive(&mut self, left: MotorGroup, right: MotorGroup) {
        self.left_drive = left;
        self.right_drive = right;
    }

    pub fn use_custom_intake(&mut self, digger: MotorGroup, rails: MotorGroup) {
        self.digger = digger;
        self.rails = rails;
    }

    pub fn use_custom_dumper(&mut self, dumper: MotorGroup) {
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

        let front_right_motor = Box::new(HoverBoardMotor::new(right_front_pwm, front_right_direction, MotorID::DriveTrainFrontRight));
        let front_left_motor = Box::new(HoverBoardMotor::new(left_front_pwm, front_left_direction, MotorID::DriveTrainFrontLeft));
        let rear_right_motor = Box::new(HoverBoardMotor::new(right_rear_pwm, rear_right_direction, MotorID::DriveTrainRearRight));
        let rear_left_motor = Box::new(HoverBoardMotor::new(left_rear_pwm, rear_left_direction, MotorID::DriveTrainRearLeft));

        self.left_drive = MotorGroup::new(vec![front_left_motor, rear_left_motor]);
        self.right_drive = MotorGroup::new(vec![front_right_motor, rear_right_motor]);
    }

    pub fn new() -> Self {
        let left_motor = Box::new(PrintMotor::new("Left"));
        let right_motor = Box::new(PrintMotor::new("Right"));
        let digger_motor = Box::new(PrintMotor::new("Digger"));
        let rails_motor = Box::new(PrintMotor::new("Rails"));
        let dumper_motor = Box::new(PrintMotor::new("Dumper"));

        let left_group = MotorGroup::new(vec![left_motor]);
        let right_group = MotorGroup::new(vec![right_motor]);
        let digger_group = MotorGroup::new(vec![digger_motor]);
        let dumper_group = MotorGroup::new(vec![dumper_motor]);
        let rails_group = MotorGroup::new(vec![rails_motor]);

        Self {
            left_drive: left_group,
            right_drive: right_group,
            digger: digger_group,
            rails: rails_group,
            dumper: dumper_group,
        }
    }

    pub fn build(self) -> Robot {
        let (controller_sender, controller_receiver) = channel();

        // Create Robot status
        let robot_status = GlobalLifeStatus::new();

        // Create RobotView
        let robot_view = MechatronicsMessageSender::new(controller_sender, robot_status.clone());

        // Create server
        let bfr = comms::stage(robot_view);

        // Create DriveTrain
        let drive_train = DriveTrain::new(self.left_drive, self.right_drive, robot_status.clone());

        let digger = BucketLadder::new(self.digger, self.rails, robot_status.clone());

        let dumper = Dumper::new(robot_status.clone(), self.dumper);

        // Create Robot Controller
        let robot_controller = RobotController::new(controller_receiver, drive_train, dumper, digger, robot_status);

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