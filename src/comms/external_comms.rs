use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread::spawn;

use crate::comms::Communicator;
use crate::comms::CommunicatorError;
use crate::comms::SendableMessage;
use crate::framework::logging::get_timestamp;
use crate::framework::logging::LogData;
use crate::framework::logging::LogType;
use crate::subsystems::drive_train::DriveTrainCommand;
use std::io;
use std::str::SplitWhitespace;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 2401;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProtocolSubsystem {
    DriveTrain,
}

#[derive(Copy, Clone)]
enum ReceivableMessage {
    Kill,
    Revive,
    Enable(ProtocolSubsystem),
    Disable(ProtocolSubsystem),
    Drive(f32, f32),
    Brake,
}

pub struct ExternalComms {
    sending_channel: Receiver<Box<SendableMessage>>,
    logging_channel: Sender<LogData>,
    communicator: Communicator,
    drive_train_channel: Sender<DriveTrainCommand>,
}

impl ExternalComms {
    /// Instantiates the comms.
    /// This constructor will bind the listener.
    pub fn new(logging_channel: Sender<LogData>, sending_channel: Receiver<Box<SendableMessage>>,
               drive_train_channel: Sender<DriveTrainCommand>) -> ExternalComms {
        let communicator = Communicator::from(ADDRESS, PORT)
            .expect("Could not create communicator for external comms!");

        ExternalComms {
            sending_channel,
            logging_channel,
            communicator,
            drive_train_channel,
        }
    }

    /// Starts the comms in a <b>new</b> thread.
    pub fn start(mut self) {
        spawn(move || {
            loop {
                self.run();
            }
        });
    }

    fn run(&mut self) {
        self.check_connections();
        self.receive_messages();
        self.send_messages();
    }

    fn check_connections(&mut self) {
        if let Err(error) = self.communicator.check_connections() {
            match error {
                CommunicatorError::InvalidAddress => panic!("Invalid address error for check_connections! This should not be possible"),
                CommunicatorError::DisconnectedListener => self.handle_lost_listener(),
                CommunicatorError::BadRead => unimplemented!(),
            }
        }
    }

    fn send_messages(&mut self) {
        match self.sending_channel.try_recv() {
            Ok(message) => self.send_message(message),
            Err(try_error) => {
                if let TryRecvError::Disconnected = try_error {
                    self.handle_sending_channel_disconnect();
                }
            }
        }
    }

    fn receive_messages(&mut self) {
        for result in self.communicator.receive_next_lines() {
            match result {
                Ok(message) => self.handle_message(message),
                Err(error) => {}
            }
        }
    }

    fn handle_message(&mut self, message: String) {
        let parsed_result = parse_message(message);
        unimplemented!()
    }

    fn handle_lost_listener(&mut self) {
        unimplemented!()
    }

    fn send_message(&mut self, message: Box<SendableMessage>) {
        let sending_string = message.encode();

        self.communicator.send_line(sending_string).expect("Error in sending a line!");
    }

    fn handle_sending_channel_disconnect(&mut self) {
        let timestamp = get_timestamp();
        let severity = LogType::Fatal;
        let description = "Sending channel disconnected in external comms!";
        let log = LogData::new(severity, timestamp, description.to_string());
        self.logging_channel.send(log).expect("Sending and logging channels disconnected in external comms");
        panic!("{}", "Sending channel disconnected in external comms!");
    }
}

fn parse_message(message: String) -> Result<ReceivableMessage, LogData> {
    let message_clone = message.clone();
    let mut elements = message_clone.split_whitespace();
    let command = match elements.next() {
        Some(com) => com,
        None => {
            unimplemented!()
        },
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

fn parse_drive_command(original_message: String, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.count().clone() != 2 as usize {
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

fn parse_enable_command(original_message: String, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.count() != 1 {
        let log = get_wrong_arg_count_log(original_message, 1, args.count() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args)?;
        Ok(ReceivableMessage::Enable(parsed_subsystem))
    }
}

fn parse_disable_command(original_message: String, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.count() != 1 {
        let log = get_wrong_arg_count_log(original_message, 1, args.count() as u64);
        Err(log)
    } else {
        let parsed_subsystem = parse_subsystem(args)?;
        Ok(ReceivableMessage::Disable(parsed_subsystem))
    }
}

fn parse_subsystem(mut args: SplitWhitespace) -> Result<ProtocolSubsystem, LogData> {
    unimplemented!()
}

fn parse_revive_command(original_message: String, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.count() != 0 {
        let log = get_wrong_arg_count_log(original_message, 0, args.count() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Revive)
    }
}

fn parse_kill_command(original_message: String, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.count() != 0 {
        let log = get_wrong_arg_count_log(original_message, 0, args.count() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Kill)
    }
}

fn parse_brake_command(original_message: String, mut args: SplitWhitespace) -> Result<ReceivableMessage, LogData> {
    if args.count() != 0 {
        let log = get_wrong_arg_count_log(original_message, 0, args.count() as u64);
        Err(log)
    } else {
        Ok(ReceivableMessage::Brake)
    }
}

fn get_wrong_arg_count_log(message: String, expected: u64, actual: u64) -> LogData {
    let description = format!(
        "Wrong number of elements in message '{}'. Expected {} args, instead got {}!",
        message, expected, actual);

    LogData::warning(description.as_str())
}
