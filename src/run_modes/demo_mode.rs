use std::sync::mpsc::channel;
use std::thread::spawn;

use crate::comms::driver_station::DriverStationComms;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::print_motor::PrintMotor;
use crate::drive_train::DriveTrain;
use crate::framework::Runnable;
use crate::logging::log_manager::LogManager;

pub fn run_demo_mode() {
    let (drive_sender, drive_receiver) = channel();
    let (comms_sender, comms_receiver) = channel();

    let mut logger = LogManager::new("./RMC_Logs", 16);
    let log_sender = logger.get_sender();

    let comms = DriverStationComms::new(log_sender.clone(), comms_receiver, drive_sender.clone());

    let left_back = Box::new(PrintMotor::new("LB"));
    let left_front = Box::new(PrintMotor::new("LF"));
    let right_back = Box::new(PrintMotor::new("RB"));
    let right_front = Box::new(PrintMotor::new("RF"));

    let left_side = Box::new(MotorGroup::new(vec![left_back, left_front]));
    let right_side = Box::new(MotorGroup::new(vec![right_back, right_front]));

    let mut drive_train = DriveTrain::new(drive_receiver, log_sender.clone(), left_side, right_side);

    let logger_thread = spawn(move || logger.start());
    let _ = spawn(move || comms.start());
    let _ = spawn(move || drive_train.start());

    logger_thread.join().expect("Logging thread crashed!");
}