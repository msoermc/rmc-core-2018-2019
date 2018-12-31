use crate::comms::command_io_controller::CommandIoController;
use crate::comms::io::IoServerManager;
use crate::comms::driver_station::DriverStationInterface;
use crate::comms::driver_station::commands::create_command_parser;

pub fn create_driver_station_comms<RI, IO>(interface: RI, io_manager: IO) -> CommandIoController<RI, IO>
    where RI: DriverStationInterface, IO: IoServerManager
{
    let parser = create_command_parser();
    CommandIoController::new(parser, interface, io_manager)
}