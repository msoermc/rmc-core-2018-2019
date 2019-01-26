use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::thread::spawn;

use crate::comms;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::framework::Runnable;
use crate::mechatronics::controller::RobotController;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::MechatronicsMessageSender;
use crate::mechatronics::RobotLifeStatus;

pub mod demo_mode;
pub mod run_drive_train;

fn run_with_motors(left_group: MotorGroup, right_group: MotorGroup) {
    let (controller_sender, controller_receiver) = channel();

    // Create Robot status
    let robot_status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

    // Create RobotView
    let robot_view = MechatronicsMessageSender::new(controller_sender, robot_status.clone());

    // Create server
    let (server_sender, bfr) = comms::stage(robot_view);

    // Create DriveTrain
    let drive_train = DriveTrain::new(left_group, right_group, robot_status.clone());

    // Create Robot Controller
    let mut robot_controller = RobotController::new(server_sender.clone(), controller_receiver, drive_train, robot_status);

    // Create threads
    let controller_thread = spawn(move || robot_controller.start());
    let _rocket_thread = spawn(move || bfr.launch());

    controller_thread.join().expect("Controller thread panicked!");
}