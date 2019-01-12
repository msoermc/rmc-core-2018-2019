use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::thread::spawn;

use crate::comms::CommsView;
use crate::comms::driver_station::ConcreteDriverStationController;
use crate::comms::driver_station::factories::create_driver_station_comms;
use crate::comms::io::IoServerManager;
use crate::comms::io::tcp::TcpServerManager;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::framework::Runnable;
use crate::logging::log_manager::LogManager;
use crate::logging::log_sender::LogSender;
use crate::robot_control::controller::RobotController;
use crate::robot_control::drive_train::DriveTrain;
use crate::robot_control::RobotLifeStatus;
use crate::robot_control::RobotView;
use crate::robot_map::ADDRESS;
use crate::robot_map::LOG_PATH;
use crate::robot_map::PORT;

pub mod demo_mode;
pub mod run_drive_train;

fn run_with_motors(left_group: MotorGroup, right_group: MotorGroup) {
    // Create channels
    let (ds_sender, ds_receiver) = channel();
    let (controller_sender, controller_receiver) = channel();
    let (log_sender, log_receiver) = channel();

    // Create LoggerView
    let logger_view = LogSender::new(log_sender);

    // Create Robot status
    let robot_status = Arc::new(RwLock::new(RobotLifeStatus::Alive));

    // Create RobotView
    let robot_view = RobotView::new(controller_sender, robot_status.clone());

    // Create DS IO Driver
    let ds_io = TcpServerManager::create(ADDRESS, PORT);

    // Create DS Controller
    let ds_controller = ConcreteDriverStationController::new(robot_view, logger_view.clone(), ds_receiver);

    // Create DS View
    let comms_view = CommsView::new(ds_sender);

    // Create DS Comms
    let mut ds_comms = create_driver_station_comms(ds_controller, ds_io);

    // Create DriveTrain
    let drive_train = DriveTrain::new(left_group, right_group, robot_status.clone());

    // Create Robot Controller
    let mut robot_controller = RobotController::new(logger_view, comms_view.clone(), controller_receiver, drive_train, robot_status);

    // Create logger
    let mut logger = LogManager::new(LOG_PATH, 16, log_receiver);

    // Create threads
    let _logging_thread = spawn(move || logger.start());
    let controller_thread = spawn(move || robot_controller.start());
    let _ds_thread = spawn(move || ds_comms.start());

    controller_thread.join().expect("Controller thread panicked!");
}