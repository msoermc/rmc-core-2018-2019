use std::sync::mpsc::channel;
use crate::comms::driver_station::factories::create_driver_station_comms;
use crate::comms::io::tcp::TcpServerManager;
use crate::comms::io::IoServerManager;
use crate::comms::driver_station::ConcreteDriverStationController;
use crate::logging::log_sender::LogSender;
use crate::devices::motor_controllers::print_motor::PrintMotor;
use crate::devices::motor_controllers::motor_group::MotorGroup;
use crate::logging::log_manager::LogManager;
use std::thread::spawn;
use crate::framework::Runnable;
use std::sync::Arc;
use std::sync::RwLock;
use crate::operations::interface::ConcreteTankDriveInterface;

const ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 2401;

pub fn run_demo_mode() {
    let life = Arc::new(RwLock::new(true));
    let (log_sender, log_receiver) = channel();
    let (ds_sender, ds_receiver) = channel();
    let (drive_sender, drive_receiver) = channel();

    let log_sender = LogSender::new(log_sender);
    let drive_sender = ConcreteTankDriveInterface::new(drive_sender);

    let mut logger = LogManager::new("./RMC_Logs", 16, log_receiver);

    let ds_io_manager = TcpServerManager::create(ADDRESS, PORT);
    let ds_controller = ConcreteDriverStationController::new(Box::new(drive_sender.clone()), log_sender.clone(), ds_receiver, life.clone());

    let mut ds_comms = create_driver_station_comms(ds_controller, ds_io_manager);

    let left_back = Box::new(PrintMotor::new("LB"));
    let left_front = Box::new(PrintMotor::new("LF"));
    let right_back = Box::new(PrintMotor::new("RB"));
    let right_front = Box::new(PrintMotor::new("RF"));

    let left_side = Box::new(MotorGroup::new(vec![left_back, left_front]));
    let right_side = Box::new(MotorGroup::new(vec![right_back, right_front]));

    //let mut drive_train = DriveTrain::new(drive_receiver, log_sender.clone(), left_side, right_side, life.clone());

    let logger_thread = spawn(move || logger.start());
    let _ = spawn(move || ds_comms.start());
    //let _ = spawn(move || drive_train.start());

    logger_thread.join().expect("Logging thread crashed!");
}