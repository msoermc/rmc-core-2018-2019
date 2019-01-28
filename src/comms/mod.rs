use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::Rocket;
use rocket::State;
use crate::mechatronics::MechatronicsMessageSender;

#[cfg(test)]
mod tests;

/// A `SendableMessage` is an object that can be encoded as a message and sent off to another device.
pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

/// The `ServerSender` is a view into a `RobotCommunicator` that other threads/objects
/// can use to request that messages be sent.
#[derive(Clone, Debug)]
pub struct ServerSender {
    channel: Sender<Box<SendableMessage>>,
}

impl ServerSender {
    /// Sends a message to the remote receiver and returns `Err(LogData)` if the channel hangs up.
    pub fn send_message(&self, message: Box<SendableMessage>) {
        self.channel.send(message).expect("Failed to send message!");
    }

    /// Constructs a new `ServerSender`
    fn new(channel: Sender<Box<SendableMessage>>) -> Self {
        Self {
            channel
        }
    }
}

struct ServerState {
    receiver: Mutex<Receiver<Box<SendableMessage>>>,
    robot_controller: Mutex<MechatronicsMessageSender>,
}

struct Drive {}

/// Launches the server
pub fn stage(robot_controller: MechatronicsMessageSender) -> (ServerSender, Rocket) {
    let (send, recv) = channel();

    let server_sender = ServerSender::new(send);

    let state = ServerState {
        receiver: Mutex::new(recv),
        robot_controller: Mutex::new(robot_controller),
    };
    let rocket = rocket::ignite()
        .manage(state)
        .mount("/",
               routes![handle_drive,
                              handle_enable_drive,
                              handle_disable_drive,
                              handle_kill,
                              handle_revive,
                              handle_brake,
                              index,
                              files]);


    (server_sender, rocket)
}

#[post("/robot/drive_train/drive/<left>/<right>")]
fn handle_drive(left: f32, right: f32, state: State<ServerState>) -> Status {
    info!("Received drive message: [{}, {}]", left, right);
    match state.robot_controller.lock() {
        Ok(controller) => if controller.drive(left, right).is_err() {
            Status::BadRequest
        } else {
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/drive_train/enable")]
fn handle_enable_drive(state: State<ServerState>) -> Status {
    info!("Received enable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.enable_drive_train();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/drive_train/disable")]
fn handle_disable_drive(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.disable_drive_train();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/dumper/enable")]
fn handle_enable_dumper(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.enable_dumper();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/dumper/disable")]
fn handle_disable_dumper(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.disable_dumper();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/dumper/dump")]
fn handle_dump(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.dump();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/dumper/reset")]
fn handle_reset_dumper(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.reset_dumper();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/dumper/stop")]
fn handle_stop_dumper(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.stop_dumper();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/digger/enable")]
fn handle_enable_digger(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.enable_ladder();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/digger/disable")]
fn handle_disable_digger(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.disable_ladder();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/digger/rails/raise")]
fn handle_raise_digger(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.raise_ladder();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/digger/rails/lower")]
fn handle_lower_digger(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.lower_ladder();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/digger/rails/stop")]
fn handle_stop_digger_rails(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.freeze_ladder_height();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/digger/dig")]
fn handle_dig(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.dig();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/digger/stop-digging")]
fn handle_stop_digging(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.stop_digger();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/kill")]
fn handle_kill(state: State<ServerState>) -> Status {
    info!("Received kill message");
    state.robot_controller.lock().unwrap().kill();
    Status::Ok

}

#[post("/robot/drive_train/brake")]
fn handle_brake(state: State<ServerState>) -> Status {
    info!("Received brake message");
    state.robot_controller.lock().unwrap().brake();
    Status::Ok
}

#[post("/robot/revive")]
fn handle_revive(state: State<ServerState>) -> Status {
    info!("Received revive message");
    state.robot_controller.lock().unwrap().revive();
    Status::Ok
}

#[get("/")]
fn index() -> Option<NamedFile> {
    info!("Received static request for index!");
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    info!("Received static request: {:?}", file);
    NamedFile::open(Path::new("static/").join(file)).ok()
}