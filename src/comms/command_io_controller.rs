use crate::comms::io::IoServerManager;
use crate::logging::LogAccepter;
use crate::comms::reading::Parser;
use crate::framework::Runnable;
use crate::comms::SendableMessage;

pub struct CommandIoController<R, I> where I: IoServerManager, R: RobotInterface {
    parser: Parser<R>,
    robot_interface: R,
    io: I,
}

impl<R, I> CommandIoController<R, I> where I: IoServerManager, R: RobotInterface {
    pub fn new(parser: Parser<R>, robot_interface: R, io: I) -> Self {
        CommandIoController {
            parser,
            robot_interface,
            io,
        }
    }

    fn check_connection_statuses(&mut self) {
        if let Err(connection_status) = self.io.check_connections() {
            self.robot_interface.accept_log(connection_status);
        }
    }

    fn receive_messages(&mut self) {
        let messages_results = self.io.receive_next_lines();

        for message_result in messages_results {
            match message_result {
                Ok(message) => {
                    match self.parser.parse(&message) {
                        Ok(command) => command.accept(&self.robot_interface),
                        Err(log) => self.robot_interface.accept_log(log),
                    }
                }
                Err(log) => self.robot_interface.accept_log(log),
            }
        }
    }

    fn send_messages(&mut self) {
        if let Some(next_message) = self.robot_interface.get_next_requested_send() {
            let encoding = next_message.encode();
            self.io.send_line(encoding);
        }
    }
}

impl<R, I> Runnable for CommandIoController<R, I> where I: IoServerManager, R: RobotInterface {
    fn init(&mut self) {
        //do nothing
    }

    fn run(&mut self) -> bool {
        self.check_connection_statuses();

        self.receive_messages();

        self.send_messages();

        true
    }
}

pub trait RobotInterface: LogAccepter {
    fn get_next_requested_send(&self) -> Option<Box<SendableMessage>>;
}