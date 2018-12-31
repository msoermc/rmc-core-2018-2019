use crate::comms::driver_station::DriverStationInterface;
use crate::comms::reading::Parser;
use crate::comms::driver_station::commands::drive_command::DriveCommandParser;
use crate::comms::driver_station::commands::brake_command::BrakeCommandParser;
use crate::comms::driver_station::commands::kill_command::KillCommandParser;
use crate::comms::driver_station::commands::revive_command::ReviveCommandParser;
use crate::comms::driver_station::commands::enable_command::EnableCommandParser;
use crate::comms::driver_station::commands::disable_command::DisableCommandParser;

pub mod drive_command;
pub mod brake_command;
pub mod kill_command;
pub mod revive_command;
pub mod enable_command;
pub mod disable_command;

pub fn create_command_parser<I>() -> Parser<I> where I: DriverStationInterface {
    let mut parser = Parser::new();

    parser.add_reader("drive", Box::new(DriveCommandParser::new()));
    parser.add_reader("brake", Box::new(BrakeCommandParser::new()));
    parser.add_reader("kill", Box::new(KillCommandParser::new()));
    parser.add_reader("revive", Box::new(ReviveCommandParser::new()));
    parser.add_reader("enable", Box::new(EnableCommandParser::new()));
    parser.add_reader("disable", Box::new(DisableCommandParser::new()));

    parser
}