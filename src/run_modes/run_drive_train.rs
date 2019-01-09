use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::thread::spawn;

use crate::comms::driver_station::ConcreteDriverStationController;
use crate::comms::driver_station::factories::create_driver_station_comms;
use crate::comms::io::IoServerManager;
use crate::comms::io::tcp::TcpServerManager;
use crate::devices::create_pin;
use crate::devices::create_pwm;
use crate::devices::motor_controllers::hover_board::HoverBoardMotor;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::framework::Runnable;
use crate::logging::log_manager::LogManager;
use crate::logging::log_sender::LogSender;
use crate::robot_map::*;
use crate::operations::interface::ConcreteTankDriveInterface;



pub fn run_drive_train() {

}