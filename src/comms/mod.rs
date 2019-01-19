use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::sync::RwLock;

use rocket::http::Status;
use rocket::State;

use crate::control::DriveCommandMessage;
use crate::control::RobotView;

/// A `SendableMessage` is an object that can be encoded as a message and sent off to another device.
pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

/// The `CommsView` is a view into a `RobotCommunicator` that other threads/objects
/// can use to request that messages be sent.
#[derive(Clone, Debug)]
pub struct CommsView {
    channel: Sender<Box<SendableMessage>>,
}

impl CommsView {
    /// Sends a message to the remote receiver and returns `Err(LogData)` if the channel hangs up.
    pub fn send_message(&self, message: Box<SendableMessage>) {
        self.channel.send(message).expect("Failed to send message!");
    }

    /// Constructs a new `CommsView`
    pub fn new(channel: Sender<Box<SendableMessage>>) -> Self {
        Self {
            channel
        }
    }
}

struct CommsState {
    receiver: Mutex<Receiver<Box<SendableMessage>>>,
    robot_controller: Mutex<RobotView>,
}

pub fn launch(robot_controller: RobotView) -> CommsView {
    let (send, recv) = channel();

    let comms_view = CommsView::new(send);

    let state = CommsState {
        receiver: Mutex::new(recv),
        robot_controller: Mutex::new(robot_controller),
    };

    rocket::ignite()
        .manage(state)
        .mount("/",
               routes![handle_drive,
                              handle_enable_drive,
                              handle_disable_drive,
                              handle_kill,
                              handle_revive,
                              handle_brake])
        .launch();

    comms_view
}

#[post("/drive/<left>/<right>")]
fn handle_drive(left: f32, right: f32, state: State<CommsState>) -> Status {
    if state.robot_controller.lock().unwrap().drive(left, right).is_err() {
        Status::BadRequest
    } else {
        Status::Ok
    }
}

#[post("/enable/drive_train")]
fn handle_enable_drive(state: State<CommsState>) -> Status {
    if state.robot_controller.lock().unwrap().enable_drive_train().is_err() {
        Status::BadRequest
    } else {
        Status::Ok
    }
}

#[post("/disable/drive_train")]
fn handle_disable_drive(state: State<CommsState>) -> Status {
    if state.robot_controller.lock().unwrap().disable_drive_train().is_err() {
        Status::BadRequest
    } else {
        Status::Ok
    }
}

#[post("/kill")]
fn handle_kill(state: State<CommsState>) -> Status {
    if state.robot_controller.lock().unwrap().kill().is_err() {
        Status::BadRequest
    } else {
        Status::Ok
    }
}

#[post("/revive")]
fn handle_revive(state: State<CommsState>) -> Status {
    if state.robot_controller.lock().unwrap().revive().is_err() {
        Status::BadRequest
    } else {
        Status::Ok
    }
}

#[post("/brake")]
fn handle_brake(state: State<CommsState>) -> Status {
    if state.robot_controller.lock().unwrap().brake().is_err() {
        Status::BadRequest
    } else {
        Status::Ok
    }
}