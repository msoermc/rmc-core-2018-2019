use crate::comms::command_io_controller::CommandIoController;
use crate::comms::io::IoServerManager;
use crate::comms::command_io_controller::RobotInterface;

pub fn create_driver_station_comms<RI, IO>(interface: RI, io_manager: IO) -> CommandIoController<RI, IO>
    where RI: RobotInterface, IO: IoServerManager
{
    unimplemented!()
}