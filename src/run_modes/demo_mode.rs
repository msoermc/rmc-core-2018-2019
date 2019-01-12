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
use crate::devices::motor_controllers::print_motor::PrintMotor;
use crate::framework::Runnable;
use crate::logging::log_manager::LogManager;
use crate::logging::log_sender::LogSender;
use crate::robot_control::controller::RobotController;
use crate::robot_control::drive_train::DriveTrain;
use crate::robot_control::RobotLifeStatus;
use crate::robot_control::RobotView;
use crate::robot_map::*;
use crate::run_modes::run_with_motors;

pub fn run_demo_mode() {
    let left_motor = Box::new(PrintMotor::new("Left"));
    let right_motor = Box::new(PrintMotor::new("Right"));

    let left_group = MotorGroup::new(vec![left_motor]);
    let right_group = MotorGroup::new(vec![right_motor]);

    run_with_motors(left_group, right_group);
}