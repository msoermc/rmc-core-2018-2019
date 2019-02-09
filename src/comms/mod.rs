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

/// Contains all of the state elements used by the server.
struct ServerState {
    /// A sender object which will send messages to the mechatronics controller.
    messager: Mutex<MechatronicsMessageSender>,

    /// A struct containing the entire state of the robot and all of it's component systems.
    state: Arc<GlobalRobotState>,
}

/// Prepares the server for launch.
pub fn stage(sender: MechatronicsMessageSender, state: Arc<GlobalRobotState>) -> Rocket {
    let state = ServerState {
        messager: Mutex::new(sender),
        state,
    };
    rocket::ignite()
        .manage(state)
        .mount("/",
               routes![handle_drive,
                              get_state,
                              kill,
                              revive,
                              brake,
                              dig,
                              dump,
                              lower_digger,
                              raise_digger,
                              reset_dumper,
                              stop_digger,
                              stop_dumper,
                              stop_rails,
                              switch_mode,
                              index,
                              files])
}

/// Responds with the current state of the robot, as a JSON object.
///
/// Example:
/// ```javascript
/// {
///     "life": {
///         "life": true
///     },
///     "drive": {
///         "enabled": false,
///         "left": {
///             "speed": 0
///         },
///         "right": {
///             "speed": 0
///         }
///     },
///     "dumper": {
///         "enabled": false,
///         "motor": {
///             "speed": 0
///         }
///     },
///     "intake": {
///         "left_actuator": {
///             "upper": false,
///             "lower": false,
///             "motor": {
///                 "speed": 0
///             }
///         },
///         "right_actuator": {
///             "upper": false,
///             "lower": false,
///             "motor": {
///                 "speed": 0
///             }
///         },
///         "ladder": {
///             "motor": {
///                 "speed": 0
///             }
///         },
///         "enabled": false
///     }
/// }
/// ```
#[get("/robot/state")]
fn get_state(state: State<ServerState>) -> Json<RobotStateInstance> {
    Json(state.state.get_current_state())
}

/// Switches the current mode of the robot.
/// The allowed modes are `dig`, `dump`, and `drive`.
/// When we switch to a mode, only that subsystem is enabled and the others will be disabled.
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

/// Runs the drive train at the provided speeds.
#[post("/robot/drive_train/drive/<left>/<right>")]
fn handle_drive(left: f32, right: f32, state: State<ServerState>) -> Status {
    if state.messager.lock().unwrap().drive(left, right).is_err() {
        Status::BadRequest
    } else {
        Status::Ok
    }
}

/// Starts the dumper.
#[post("/robot/dumper/dump")]
fn dump(state: State<ServerState>) {
    state.messager.lock().unwrap().dump();
}

/// Resets the dumper
#[post("/robot/dumper/reset")]
fn reset_dumper(state: State<ServerState>) {
    state.messager.lock().unwrap().reset_dumper();
}

/// Stops the dumper.
#[post("/robot/dumper/stop")]
fn stop_dumper(state: State<ServerState>) {
    state.messager.lock().unwrap().stop_dumper();
}

/// Raises the actuators on the digger.
#[post("/robot/intake/rails/raise")]
fn raise_digger(state: State<ServerState>) {
    state.messager.lock().unwrap().raise_ladder();
}

/// Lower the actuators on the digger.
#[post("/robot/intake/rails/lower")]
fn lower_digger(state: State<ServerState>) {
    state.messager.lock().unwrap().lower_ladder();
}

/// Stops the actuators.
#[post("/robot/intake/rails/stop")]
fn stop_rails(state: State<ServerState>) {
    state.messager.lock().unwrap().stop_actuators();
}

/// Starts the digger.
#[post("/robot/intake/digger/dig")]
fn dig(state: State<ServerState>) {
    state.messager.lock().unwrap().dig();
}

/// Stops the digger.
#[post("/robot/intake/digger/stop")]
fn stop_digger(state: State<ServerState>) {
    state.messager.lock().unwrap().stop_digger();
}

/// Kills the robot. When this command is invoked, all physical motion on the robot ceases.
#[post("/robot/kill")]
fn kill(state: State<ServerState>) {
    state.messager.lock().unwrap().kill();
}

/// Causes the drive train to begin braking.
#[post("/robot/drive_train/brake")]
fn brake(state: State<ServerState>) {
    state.messager.lock().unwrap().brake();
}

/// Revives a dead robot, allowing further motion.
#[post("/robot/revive")]
fn revive(state: State<ServerState>) {
    state.messager.lock().unwrap().revive();
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