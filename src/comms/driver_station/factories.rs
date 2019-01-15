use crate::comms::driver_station::commands::create_command_parser;
use crate::comms::driver_station::DriverStationController;
use crate::comms::io::IoServerManager;
use crate::comms::RobotCommunicator;
'
/// Factory method for creating a driver station controller.
pub fn create_driver_station_comms<C, I>(controller: C, io_manager: I) -> RobotCommunicator<C, I>
    where C: DriverStationController, I: IoServerManager
{
    let parser = create_command_parser();
    RobotCommunicator::new(parser, controller, io_manager)
}