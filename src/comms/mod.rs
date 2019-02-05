use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::Rocket;
use rocket::State;
use crate::mechatronics::MechatronicsMessageSender;

#[cfg(test)]
mod tests;

struct ServerState {
    robot_controller: Mutex<MechatronicsMessageSender>,
}

struct Drive {}

/// Launches the server
pub fn stage(robot_controller: MechatronicsMessageSender) -> Rocket {
    let state = ServerState {
        robot_controller: Mutex::new(robot_controller),
    };
    rocket::ignite()
        .manage(state)
        .mount("/",
               routes![handle_drive,
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

#[post("/robot/modes/<mode>")]
fn switch_mode(mode: String,state: State<ServerState>) -> Status {
    match state.robot_controller.lock() {
        Ok(controller) => {
            match mode.as_str() {
                "dig" => controller.switch_to_dig(),
                "dump" => controller.switch_to_dump(),
                "drive" => controller.switch_to_drive(),
                _ => return Status::BadRequest,
            }

            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/drive_train/drive/<left>/<right>")]
fn handle_drive(left: f32, right: f32, state: State<ServerState>) -> Status {
    match state.robot_controller.lock() {
        Ok(controller) => if controller.drive(left, right).is_err() {
            Status::BadRequest
        } else {
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/dumper/dump")]
fn handle_dump(state: State<ServerState>) -> Status {
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
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.stop_dumper();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/intake/rails/raise")]
fn handle_raise_digger(state: State<ServerState>) -> Status {
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.raise_ladder();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/intake/rails/lower")]
fn handle_lower_digger(state: State<ServerState>) -> Status {
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.lower_ladder();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/intake/rails/stop")]
fn handle_stop_rails(state: State<ServerState>) -> Status {
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.stop_actuators();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/intake/digger/dig")]
fn handle_dig(state: State<ServerState>) -> Status {
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.dig();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/robot/intake/digger/stop")]
fn handle_stop_digger(state: State<ServerState>) -> Status {
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
    state.robot_controller.lock().unwrap().kill();
    Status::Ok

}

#[post("/robot/drive_train/brake")]
fn handle_brake(state: State<ServerState>) -> Status {
    state.robot_controller.lock().unwrap().brake();
    Status::Ok
}

#[post("/robot/revive")]
fn handle_revive(state: State<ServerState>) -> Status {
    state.robot_controller.lock().unwrap().revive();
    Status::Ok
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}