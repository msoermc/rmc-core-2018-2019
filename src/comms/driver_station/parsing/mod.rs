use crate::comms::get_wrong_arg_count_log;
use crate::logging::log_data::LogData;
use crate::comms::MessageParser;
use std::collections::HashMap;
use std::num::ParseFloatError;
use crate::comms::CommandMessageParser;
use crate::comms::Command;
use crate::drive_train::DriveTrainCommand;
use std::sync::mpsc::Sender;

#[cfg(test)]
mod tests;

fn create_drive_train_parser() -> Box<MessageParser<ReceivableMessage>> {
    let mut parser = CommandMessageParser::new();

    parser.add_command(Box::new(DriveCommand::new()));

    parser
}

pub struct DriveCommand {}

impl Command<ReceivableMessage::Drive()> for DriveCommand {
    fn get_command(&self) -> &str {
        "drive"
    }

    fn parse(&self, args: &[&str]) -> Result<ReceivableMessage, LogData> {
        let left_parse_result: Result<f32, ParseFloatError> = args[1].parse();
        let right_parse_result: Result<f32, ParseFloatError> = args[1].parse();

        if let (Ok(left_speed), Ok(right_speed)) = (left_parse_result, right_parse_result) {
            if left_speed > 1.0 || right_speed > 1.0 {
                Err(LogData::warning("Received speed > 1 in drive train message"))
            } else {
                Ok(ReceivableMessage::Drive(left_speed, right_speed))
            }
        } else {
            Err(LogData::warning("Received unparseable speed in drive message"))
        }
    }

    fn handle(&mut self, command: ReceivableMessage::Drive()) {}
}

impl DriveCommand {
    fn new() -> Self {
        DriveCommand {}
    }
}

fn parse_enable_command(original_message: &str, args: &[&str]) -> Result<ReceivableMessage, LogData> {
    if args.len() != 2 {
        let log = get_wrong_arg_count_log(original_message, 1, args.len() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args[1])?;
        Ok(ReceivableMessage::Enable(parsed_subsystem))
    }
}

fn parse_disable_command(original_message: &str, args: &[&str]) -> Result<ReceivableMessage, LogData> {
    if args.len() != 2 {
        let log = get_wrong_arg_count_log(original_message, 1, args.len() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args[1])?;
        Ok(ReceivableMessage::Disable(parsed_subsystem))
    }
}

fn parse_subsystem(field: &str) -> Result<Subsystem, LogData> {
    match field {
        "drive_train" => Ok(Subsystem::DriveTrain),
        _ => Err(LogData::warning("Unrecognized subsystem in message!"))
    }
}

fn parse_revive_command(original_message: &str, args: &[&str]) -> Result<ReceivableMessage, LogData> {
    if args.len() != 1 {
        let log = get_wrong_arg_count_log(original_message, 0, args.len() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Revive)
    }
}

fn parse_kill_command(original_message: &str, args: &[&str]) -> Result<ReceivableMessage, LogData> {
    if args.len() != 1 {
        let log = get_wrong_arg_count_log(original_message, 0, args.len() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Kill)
    }
}

fn parse_brake_command(original_message: &str, args: &[&str]) -> Result<ReceivableMessage, LogData> {
    if args.len() != 1 {
        let log = get_wrong_arg_count_log(original_message, 0, args.len() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Brake)
    }
}