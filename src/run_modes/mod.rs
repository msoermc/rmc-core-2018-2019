use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::thread::spawn;

use crate::comms::CommsView;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::framework::Runnable;
use crate::robot_map::ADDRESS;
use crate::robot_map::LOG_PATH;
use crate::robot_map::PORT;
use crate::control::RobotLifeStatus;
use crate::control::RobotView;
use crate::control::drive_train::DriveTrain;
use crate::control::controller::RobotController;
use crate::comms;
use std::env::args;

pub mod demo_mode;
pub mod run_drive_train;

fn run_with_motors(left_group: MotorGroup, right_group: MotorGroup) {
    let (controller_sender, controller_receiver) = channel();

    // Create Robot status
    let robot_status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

    // Create RobotView
    let robot_view = RobotView::new(controller_sender, robot_status.clone());

    // Create comms
    let comms_view = comms::launch(robot_view);

    // Create DriveTrain
    let drive_train = DriveTrain::new(left_group, right_group, robot_status.clone());

    // Create Robot Controller
    let mut robot_controller = RobotController::new(comms_view.clone(), controller_receiver, drive_train, robot_status);

    // Create threads
    let controller_thread = spawn(move || robot_controller.start());

    controller_thread.join().expect("Controller thread panicked!");
}