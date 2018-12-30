use crate::comms::Command;
use crate::logging::log_sender::LogSender;

mod parsers;
mod commands;

enum SubsystemIdentifier {
    DriveTrainIdentifier,
}

struct DriverStationController {
    log_sender: LogSender,

}

trait DriveTrainCommand: Command<DriverStationController> {

}

trait DriveCommandReader