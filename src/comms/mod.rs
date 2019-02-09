use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::Rocket;
use rocket::State;
use rocket_contrib::json::Json;

use crate::mechatronics::MechatronicsMessageSender;
use crate::status::robot_state::GlobalRobotState;
use crate::status::robot_state::RobotStateInstance;

#[cfg(test)]
mod tests;

struct ServerState {
    messager: Mutex<MechatronicsMessageSender>,
    state: Arc<GlobalRobotState>,
}

struct Drive {}

/// Launches the server
pub fn stage(messager: MechatronicsMessageSender, state: Arc<GlobalRobotState>) -> Rocket {
    let state = ServerState {
        messager: Mutex::new(messager),
        state,
    };
    rocket::ignite()
        .manage(state)
        .mount("/",
               routes![handle_drive,
                              get_state,
                              handle_kill,
                              handle_revive,
                              handle_brake,
                              handle_dig,
                              handle_dump,
                              handle_lower_digger,
                              handle_raise_digger,
                              handle_reset_dumper,
                              handle_stop_digger,
                              handle_stop_dumper,
                              handle_stop_rails,
                              switch_mode,
                              index,
                              files])
}

#[get("/robot/state")]
fn get_state(state: State<ServerState>) -> Json<RobotStateInstance> {
    Json(state.state.get_current_state())
}


#[post("/robot/modes/<mode>")]
fn switch_mode(mode: String, state: State<ServerState>) -> Status {
    let controller = state.messager.lock().unwrap();
    match mode.as_str() {
        "dig" => controller.switch_to_dig(),
        "dump" => controller.switch_to_dump(),
        "drive" => controller.switch_to_drive(),
        _ => return Status::BadRequest,
    }

    Status::Ok
}

#[post("/robot/drive_train/drive/<left>/<right>")]
fn handle_drive(left: f32, right: f32, state: State<ServerState>) -> Status {
    if state.messager.lock().unwrap().drive(left, right).is_err() {
        Status::BadRequest
    } else {
        Status::Ok
    }
}


#[post("/robot/dumper/dump")]
fn handle_dump(state: State<ServerState>) {
    state.messager.lock().unwrap().dump();
}

#[post("/robot/dumper/reset")]
fn handle_reset_dumper(state: State<ServerState>) {
    state.messager.lock().unwrap().reset_dumper();
}

#[post("/robot/dumper/stop")]
fn handle_stop_dumper(state: State<ServerState>) {
    state.messager.lock().unwrap().stop_dumper();
}

#[post("/robot/intake/rails/raise")]
fn handle_raise_digger(state: State<ServerState>) {
    state.messager.lock().unwrap().raise_ladder();
}

#[post("/robot/intake/rails/lower")]
fn handle_lower_digger(state: State<ServerState>) {
    state.messager.lock().unwrap().lower_ladder();
}

#[post("/robot/intake/rails/stop")]
fn handle_stop_rails(state: State<ServerState>) {
    state.messager.lock().unwrap().stop_actuators();
}

#[post("/robot/intake/digger/dig")]
fn handle_dig(state: State<ServerState>) {
    state.messager.lock().unwrap().dig();
}

#[post("/robot/intake/digger/stop")]
fn handle_stop_digger(state: State<ServerState>) {
    state.messager.lock().unwrap().stop_digger();
}

#[post("/robot/kill")]
fn handle_kill(state: State<ServerState>) {
    state.messager.lock().unwrap().kill();
}

#[post("/robot/drive_train/brake")]
fn handle_brake(state: State<ServerState>) {
    state.messager.lock().unwrap().brake();
}

#[post("/robot/revive")]
fn handle_revive(state: State<ServerState>) {
    state.messager.lock().unwrap().revive();
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}