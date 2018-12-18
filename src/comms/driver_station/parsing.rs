use std::str::SplitWhitespace;

use crate::comms::get_wrong_arg_count_log;
use crate::framework::logging::LogData;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ProtocolSubsystem {
    DriveTrain,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ReceivableMessage {
    Kill,
    Revive,
    Enable(ProtocolSubsystem),
    Disable(ProtocolSubsystem),
    Drive(f32, f32),
    Brake,
}


pub fn parse_message(message: &str) -> Result<ReceivableMessage, LogData> {
    let elements: Vec<&str> = message.split_whitespace().collect();
    let command = match elements.first() {
        Some(com) => *com,
        None => {
            return Err(LogData::warning("Empty message in DS Comms!"));
        }
    };

    match command {
        "drive" => parse_drive_command(message, elements),

        "enable" => parse_enable_command(message, elements),

        "disable" => parse_disable_command(message, elements),

        "kill" => parse_kill_command(message, elements),

        "revive" => parse_revive_command(message, elements),

        "brake" => parse_brake_command(message, elements),

        _ => {
            let description = format!("Received nonexistent command, message is '{}'", message);
            Err(LogData::warning(description.as_str()))
        },
    }
}


fn parse_drive_command(original_message: &str, args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 3 as usize {
        let log = get_wrong_arg_count_log(original_message, 2, args.len() as u64);
        Err(log)
    } else {
        // It should not be possible here for us to have too few arguments since we already
        // checked our that, so it should be safe to unwrap
        let left_speed_string = args[1];
        let right_speed_string = args[2];

        let left_speed: f32 = match left_speed_string.parse() {
            Ok(speed) => speed,
            Err(_) => {
                let log = LogData::warning("Left speed not parsable!");
                return Err(log);
            }
        };

        let right_speed: f32 = match right_speed_string.parse() {
            Ok(speed) => speed,
            Err(_) => {
                let log = LogData::warning("Right speed not parsable!");
                return Err(log);
            }
        };

        Ok(ReceivableMessage::Drive(left_speed, right_speed))
    }
}

fn parse_enable_command(original_message: &str, args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 2 {
        let log = get_wrong_arg_count_log(original_message, 1, args.len() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args[1])?;
        Ok(ReceivableMessage::Enable(parsed_subsystem))
    }
}

fn parse_disable_command(original_message: &str, args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 2 {
        let log = get_wrong_arg_count_log(original_message, 1, args.len() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args[1])?;
        Ok(ReceivableMessage::Disable(parsed_subsystem))
    }
}

fn parse_subsystem(field: &str) -> Result<ProtocolSubsystem, LogData> {
    match field {
        "drive_train" => Ok(ProtocolSubsystem::DriveTrain),
        _ => Err(LogData::warning("Unrecognized subsystem in message!"))
    }
}

fn parse_revive_command(original_message: &str, args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 1 {
        let log = get_wrong_arg_count_log(original_message, 0, args.len() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Revive)
    }
}

fn parse_kill_command(original_message: &str, args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 1 {
        let log = get_wrong_arg_count_log(original_message, 0, args.len() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Kill)
    }
}

fn parse_brake_command(original_message: &str, args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 1 {
        let log = get_wrong_arg_count_log(original_message, 0, args.len() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Brake)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive_parsing() {
        let valid_f_f = parse_message("drive 2.0 1.0").unwrap();
        let valid_i_i = parse_message("drive 2 1").unwrap();
        let valid_i_f = parse_message("drive 2 1.0").unwrap();
        let valid_f_i = parse_message("drive 2.0 1").unwrap();
        let invalid = parse_message("drive 2 3 4");
        let invalid_bad_left = parse_message("drive hi 4");
        let invalid_bad_right = parse_message("drive 5 hi");
        let invalid_bad_args = parse_message("drive hi bye");


        let expected = ReceivableMessage::Drive(2.0, 1.0);

        assert_eq!(valid_f_f, expected);
        assert_eq!(valid_i_i, expected);
        assert_eq!(valid_i_f, expected);
        assert_eq!(valid_f_i, expected);
        assert!(invalid.is_err());
        assert!(invalid_bad_left.is_err());
        assert!(invalid_bad_right.is_err());
        assert!(invalid_bad_args.is_err());
    }

    #[test]
    fn test_kill() {
        let valid = parse_message("kill").unwrap();
        let invalid = parse_message("kill hi");
        let expected_valid = ReceivableMessage::Kill;

        assert_eq!(valid, expected_valid);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_revive() {
        let valid = parse_message("revive").unwrap();
        let invalid = parse_message("revive hi");
        let expected_valid = ReceivableMessage::Revive;

        assert_eq!(valid, expected_valid);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_brake() {
        let valid = parse_message("brake").unwrap();
        let invalid = parse_message("brake hi");
        let expected_valid = ReceivableMessage::Brake;

        assert_eq!(valid, expected_valid);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_enable() {
        let valid = parse_message("enable drive_train").unwrap();
        let invalid_no_args = parse_message("enable");
        let invalid_too_many_args = parse_message("enable drive_train hi");
        let invalid_bad_subsystem = parse_message("enable fail");
        let expected_valid = ReceivableMessage::Enable(ProtocolSubsystem::DriveTrain);

        assert_eq!(valid, expected_valid);
        assert!(invalid_no_args.is_err());
        assert!(invalid_too_many_args.is_err());
        assert!(invalid_bad_subsystem.is_err());
    }

    #[test]
    fn test_disable() {
        let valid = parse_message("disable drive_train").unwrap();
        let invalid_no_args = parse_message("disable");
        let invalid_too_many_args = parse_message("disable drive_train hi");
        let invalid_bad_subsystem = parse_message("disable fail");
        let expected_valid = ReceivableMessage::Disable(ProtocolSubsystem::DriveTrain);

        assert_eq!(valid, expected_valid);
        assert!(invalid_no_args.is_err());
        assert!(invalid_too_many_args.is_err());
        assert!(invalid_bad_subsystem.is_err());
    }

    #[test]
    fn test_empty_message() {
        let actual = parse_message("").unwrap_err();
        let expected = LogData::warning("Empty message in DS Comms!");

        assert_eq!(actual.get_severity(), expected.get_severity());
        assert_eq!(actual.get_description(), expected.get_description());
    }

    #[test]
    fn test_nonexistent_command() {
        let actual_1 = parse_message("annihilate").unwrap_err();
        let actual_2 = parse_message("annihilate Noah").unwrap_err();
        let expected_1 = LogData::warning("Received nonexistent command, message is 'annihilate'");
        let expected_2 = LogData::warning("Received nonexistent command, message is 'annihilate Noah'");

        assert_eq!(actual_1.get_severity(), expected_1.get_severity());
        assert_eq!(actual_1.get_description(), expected_1.get_description());

        assert_eq!(actual_2.get_severity(), expected_2.get_severity());
        assert_eq!(actual_2.get_description(), expected_2.get_description());
    }
}