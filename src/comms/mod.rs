use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::Rocket;
use rocket::State;
use rocket_contrib::json::Json;
use serde::ser::Serialize;

use crate::main;
use crate::mechatronics::commands::RobotCommandFactory;
use crate::mechatronics::RobotMessenger;
use crate::status::robot_state::GlobalRobotState;
use crate::status::robot_state::RobotStateInstance;

#[cfg(test)]
mod tests;

/// Contains all of the state elements used by the server.
struct ServerState {
    /// A sender object which will send messages to the mechatronics controller.
    messenger: RobotMessenger,

    /// A struct containing the entire state of the robot and all of it's component systems.
    state: Arc<GlobalRobotState>,

    command_factory: RobotCommandFactory,
}

/// Prepares the server for launch.
pub fn stage(messenger: RobotMessenger, state: Arc<GlobalRobotState>, command_factory: RobotCommandFactory) -> Rocket {
    rocket::ignite()
        .manage(messenger)
        .manage(state)
        .manage(command_factory)
        .mount("/",
               routes![get_state,
                              index,
                              files,
                              favicon,
                              update_mode,
                              ])
}

#[derive(Deserialize, Serialize)]
pub enum RobotMode {
    Digging,
    Driving,
    Dumping,
}

#[post("/robot/mode", format = "application/json", data = "<mode>")]
fn update_mode(mode: Json<RobotMode>, messenger: State<RobotMessenger>, factory: State<RobotCommandFactory>) {
    match mode.into_inner() {
        RobotMode::Digging => messenger.send_command(Box::new(factory.generate_intake_switch_command())),
        RobotMode::Driving => messenger.send_command(Box::new(factory.generate_drive_switch_command())),
        RobotMode::Dumping => messenger.send_command(Box::new(factory.generate_dumper_switch_command())),
    }
}

/// Responds with the current state of the robot, as a JSON object.
#[get("/robot")]
fn get_state(state: State<Arc<GlobalRobotState>>) -> Json<RobotStateInstance> {
    Json(state.get_current_state())
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("favicon.png")).ok()
}

/// Retrieves the index.html file
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

/// Retrieves a file from the /static/ directory.
#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}