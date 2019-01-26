use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::thread::spawn;

use rocket::Rocket;
use slog::Logger;
use sysfs_gpio::Pin;
use sysfs_pwm::Pwm;

use crate::comms;
use crate::devices::enable_pins;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::print_motor::PrintMotor;
use crate::devices::motor_controllers::pwm::PwmMotor;
use crate::framework::Runnable;
use crate::mechatronics::controller::RobotController;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::MechatronicsMessageSender;
use crate::mechatronics::RobotLifeStatus;
use crate::robot_map::*;
use rocket::local::Client;

pub struct RobotBuilder {
    left_drive: Option<MotorGroup>,
    right_drive: Option<MotorGroup>,
}

impl RobotBuilder {
    pub fn add_drive_groups(&mut self, left: MotorGroup, right: MotorGroup) {
        self.left_drive = Some(left);
        self.right_drive = Some(right);
    }

    pub fn add_real_drive(&mut self) {
        enable_pins().expect("Failed to enable pins!");

        let left_front_pwm = Pwm::new(FRONT_LEFT_PWM_CHIP, FRONT_LEFT_PWM_NUMBER).expect("Front left pwm");
        let left_rear_pwm = Pwm::new(REAR_LEFT_PWM_CHIP, REAR_LEFT_PWM_NUMBER).expect("Rear left pwm");
        let right_front_pwm = Pwm::new(FRONT_RIGHT_PWM_CHIP, FRONT_RIGHT_PWM_NUMBER).expect("Front right pwm");
        let right_rear_pwm = Pwm::new(REAR_RIGHT_PWM_CHIP, REAR_RIGHT_PWM_NUMBER).expect("Rear right pwm");

        let front_right_direction = Pin::new(FRONT_RIGHT_DIRECTION);
        let front_left_direction = Pin::new(FRONT_LEFT_DIRECTION);
        let rear_right_direction = Pin::new(REAR_RIGHT_DIRECTION);
        let rear_left_direction = Pin::new(REAR_LEFT_DIRECTION);

        let front_right_motor = Box::new(PwmMotor::create(right_front_pwm, front_right_direction, MotorID::DriveTrainFrontRight).expect("Front right motor"));
        let front_left_motor = Box::new(PwmMotor::create(left_front_pwm, front_left_direction, MotorID::DriveTrainFrontLeft).expect("Front left motor"));
        let rear_right_motor = Box::new(PwmMotor::create(right_rear_pwm, rear_right_direction, MotorID::DriveTrainRearRight).expect("Rear right motor"));
        let rear_left_motor = Box::new(PwmMotor::create(left_rear_pwm, rear_left_direction, MotorID::DriveTrainRearLeft).expect("Rear left motor"));

        self.left_drive = Some(MotorGroup::new(vec![front_left_motor, rear_left_motor]));
        self.right_drive = Some(MotorGroup::new(vec![front_right_motor, rear_right_motor]));
    }

    pub fn launch(self) {
        let (controller_sender, controller_receiver) = channel();

        // Create Robot status
        let robot_status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

        // Create RobotView
        let robot_view = MechatronicsMessageSender::new(controller_sender, robot_status.clone());

        // Create server
        let (server_sender, bfr) = comms::stage(robot_view);

        // Create DriveTrain
        let drive_train = match (self.left_drive, self.right_drive) {
            (Some(left), Some(right)) => DriveTrain::new(left, right, robot_status.clone()),
            _ => {
                let left_motor = Box::new(PrintMotor::new("Left"));
                let right_motor = Box::new(PrintMotor::new("Right"));

                let left_group = MotorGroup::new(vec![left_motor]);
                let right_group = MotorGroup::new(vec![right_motor]);
                DriveTrain::new(left_group, right_group, robot_status.clone())
            }
        };

        // Create Robot Controller
        let mut robot_controller = RobotController::new(server_sender.clone(), controller_receiver, drive_train, robot_status);

        // Create threads
        let controller_thread = spawn(move || robot_controller.start());
        let _rocket_thread = spawn(move || bfr.launch());

        controller_thread.join().expect("Controller thread panicked!");
    }

    pub fn launch_tester(self) -> Client {
        let (controller_sender, controller_receiver) = channel();

        // Create Robot status
        let robot_status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

        // Create RobotView
        let robot_view = MechatronicsMessageSender::new(controller_sender, robot_status.clone());

        // Create server
        let (server_sender, bfr) = comms::stage(robot_view);

        // Create DriveTrain
        let drive_train = match (self.left_drive, self.right_drive) {
            (Some(left), Some(right)) => DriveTrain::new(left, right, robot_status.clone()),
            _ => {
                let left_motor = Box::new(PrintMotor::new("Left"));
                let right_motor = Box::new(PrintMotor::new("Right"));

                let left_group = MotorGroup::new(vec![left_motor]);
                let right_group = MotorGroup::new(vec![right_motor]);
                DriveTrain::new(left_group, right_group, robot_status.clone())
            }
        };

        // Create Robot Controller
        let mut robot_controller = RobotController::new(server_sender.clone(), controller_receiver, drive_train, robot_status);

        // Create threads
        spawn(move || robot_controller.start());
        Client::new(bfr).expect("Failed to launch client!")
    }
}