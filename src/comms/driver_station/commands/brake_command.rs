use crate::comms::driver_station::DriverStationController;
use crate::comms::get_wrong_arg_count_log;
use crate::comms::parsing::Command;
use crate::comms::parsing::CommandParser;
use crate::logging::log_data::LogData;

pub struct BrakeCommand {}

pub struct BrakeCommandParser {}

impl ToString for BrakeCommand {
    fn to_string(&self) -> String {
        "brake".to_string()
    }
}

impl<I> Command<I> for BrakeCommand where I: DriverStationController {
    fn execute(&self, interface: &I) {
        interface.get_view().brake().unwrap();
    }
}

impl BrakeCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BrakeCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I> CommandParser<I> for BrakeCommandParser where I: DriverStationController {
    fn parse(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        if args.len() > 1 {
            Err(get_wrong_arg_count_log(args, 1, args.len() as u64))
        } else {
            Ok(Box::new(BrakeCommand::new()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comms::driver_station::ConcreteDriverStationController;

    use super::*;

    #[test]
    fn test_valid() {
        let input = ["brake"];
        let expected = "brake";

        let parser = BrakeCommandParser::new();

        let actual: Box<Command<ConcreteDriverStationController>> = parser.parse(&input).unwrap();

        assert_eq!(expected, &actual.to_string());
    }
}