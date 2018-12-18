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
    let mut elements: Vec<&str> = message.split_whitespace().collect();
    let command = match elements.first() {
        Some(com) => *com,
        None => {
            unimplemented!()
        }
    };

    match command {
        "drive" => parse_drive_command(message, elements),

        "enable" => parse_enable_command(message, elements),

        "disable" => parse_disable_command(message, elements),

        "kill" => parse_kill_command(message, elements),

        "revive" => parse_revive_command(message, elements),

        "brake" => parse_brake_command(message, elements),

        _ => unimplemented!(),
    }
}


fn parse_drive_command(original_message: &str, mut args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
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

fn parse_enable_command(original_message: &str, mut args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 2 {
        let log = get_wrong_arg_count_log(original_message, 1, args.len() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args[1])?;
        Ok(ReceivableMessage::Enable(parsed_subsystem))
    }
}

fn parse_disable_command(original_message: &str, mut args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
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

fn parse_revive_command(original_message: &str, mut args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 1 {
        let log = get_wrong_arg_count_log(original_message, 0, args.len() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Revive)
    }
}

fn parse_kill_command(original_message: &str, mut args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
    if args.len() != 1 {
        let log = get_wrong_arg_count_log(original_message, 0, args.len() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Kill)
    }
}

fn parse_brake_command(original_message: &str, mut args: Vec<&str>) -> Result<ReceivableMessage, LogData> {
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

        let expected = ReceivableMessage::Drive(2.0, 1.0);

        assert_eq!(valid_f_f, expected);
        assert_eq!(valid_i_i, expected);
        assert_eq!(valid_i_f, expected);
        assert_eq!(valid_f_i, expected);
    }
}