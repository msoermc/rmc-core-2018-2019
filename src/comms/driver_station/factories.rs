use crate::comms::driver_station::commands::create_command_parser;
use crate::comms::driver_station::DriverStationController;
use crate::comms::io::IoServerManager;
use crate::comms::robot_communicator::RobotCommunicator;

pub fn create_driver_station_comms<RI, IO>(interface: RI, io_manager: IO) -> RobotCommunicator<RI, IO>
    where RI: DriverStationController, IO: IoServerManager
{
    let parser = create_command_parser();
    RobotCommunicator::new(parser, interface, io_manager)
}