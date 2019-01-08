use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::thread::spawn;

use crate::comms::driver_station::ConcreteDriverStationController;
use crate::comms::driver_station::factories::create_driver_station_comms;
use crate::comms::io::IoServerManager;
use crate::comms::io::tcp::TcpServerManager;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::devices::motor_controllers::print_motor::PrintMotor;
use crate::drive_train::DriveTrain;
use crate::drive_train::interface::ConcreteTankDriveInterface;
use crate::framework::Runnable;
use crate::logging::log_manager::LogManager;
use crate::logging::log_sender::LogSender;
use crate::devices::create_pwm;
use crate::devices::create_pin;

const ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 2401;

pub fn run_drive_train() {
    let life = Arc::new(AtomicBool::new(true));
    let (log_sender, log_receiver) = channel();
    let (ds_sender, ds_receiver) = channel();
    let (drive_sender, drive_receiver) = channel();

    let log_sender = LogSender::new(log_sender);
    let drive_sender = ConcreteTankDriveInterface::new(drive_sender);

    let mut logger = LogManager::new("./RMC_Logs", 16, log_receiver);

    let ds_io_manager = TcpServerManager::create(ADDRESS, PORT);
    let ds_controller = ConcreteDriverStationController::new(Box::new(drive_sender.clone()), log_sender.clone(), ds_receiver, life);

    let mut ds_comms = create_driver_station_comms(ds_controller, ds_io_manager);

    let lb_pwm = create_pwm(1, 1).expect("Could not create Pwm!");
    let lf_pwm = create_pwm(3, 1).expect("Could not create Pwm!");
    let rb_pwm = create_pwm(6, 1).expect("Could not create Pwm!");
    let rf_pwm = create_pwm(0, 1).expect("Could not create Pwm!");

    let lb_direction = create_pin(49).expect("Could not create GPIO pin!");
    let lf_direction = create_pin(117).expect("Could not create GPIO pin!");
    let rb_direction = create_pin(115).expect("Could not create GPIO pin!");
    let rf_direction = create_pin(60).expect("Could not create GPIO pin!");

    let left_back = Box::new(HoverBoardMotor::new(lb_pwm, lb_direction));
    let left_front = Box::new(HoverBoardMotor::new(lf_pwm, lf_direction));
    let right_back = Box::new(HoverBoardMotor::new(rb_pwm, rb_direction));
    let right_front = Box::new(HoverBoardMotor::new(rf_pwm, rf_direction));

    let left_side = Box::new(MotorGroup::new(vec![left_back, left_front]));
    let right_side = Box::new(MotorGroup::new(vec![right_back, right_front]));

    let mut drive_train = DriveTrain::new(drive_receiver, log_sender.clone(), left_side, right_side);

    let logger_thread = spawn(move || logger.start());
    let _ = spawn(move || ds_comms.start());
    let _ = spawn(move || drive_train.start());

    logger_thread.join().expect("Logging thread crashed!");
}