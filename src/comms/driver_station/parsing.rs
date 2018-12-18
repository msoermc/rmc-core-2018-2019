use std::str::SplitWhitespace;

use crate::comms::get_wrong_arg_count_log;
use crate::framework::logging::LogData;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProtocolSubsystem {
    DriveTrain,
}

#[derive(Copy, Clone)]
pub enum ReceivableMessage {
    Kill,
    Revive,
    Enable(ProtocolSubsystem),
    Disable(ProtocolSubsystem),
    Drive(f32, f32),
    Brake,
}

pub fn parse_drive_command(original_message: &str, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.by_ref().count() != 2 as usize {
        let log = get_wrong_arg_count_log(original_message, 2, args.count() as u64);
        Err(log)
    } else {
        // It should not be possible here for us to have too few arguments since we already
        // checked our that, so it should be safe to unwrap
        let left_speed_string = args.next().unwrap();
        let right_speed_string = args.next().unwrap();

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

pub fn parse_enable_command(original_message: &str, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.by_ref().count() != 1 {
        let log = get_wrong_arg_count_log(original_message, 1, args.count() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args.next().unwrap())?;
        Ok(ReceivableMessage::Enable(parsed_subsystem))
    }
}

pub fn parse_disable_command(original_message: &str, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.by_ref().count() != 1 {
        let log = get_wrong_arg_count_log(original_message, 1, args.count() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args.next().unwrap())?;
        Ok(ReceivableMessage::Disable(parsed_subsystem))
    }
}

pub fn parse_subsystem(field: &str) -> Result<ProtocolSubsystem, LogData> {
    match field {
        "drive_train" => Ok(ProtocolSubsystem::DriveTrain),
        _ => Err(LogData::warning("Unrecognized subsystem in message!"))
    }
}

pub fn parse_revive_command(original_message: &str, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.by_ref().count() != 0 {
        let log = get_wrong_arg_count_log(original_message, 0, args.count() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Revive)
    }
}

pub fn parse_kill_command(original_message: &str, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.by_ref().count() != 0 {
        let log = get_wrong_arg_count_log(original_message, 0, args.count() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Kill)
    }
}

pub fn parse_brake_command(original_message: &str, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.by_ref().count() != 0 {
        let log = get_wrong_arg_count_log(original_message, 0, args.count() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Brake)
    }
}