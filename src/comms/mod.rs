use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::Rocket;
use rocket::State;
use rocket_contrib::json::Json;

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
    let state = ServerState {
        messenger,
        state,
        command_factory,
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
    match mode.as_str() {
        "dig" => state.messenger.send_command(Box::new(state.command_factory.generate_intake_switch_command())),
        "dump" => state.messenger.send_command(Box::new(state.command_factory.generate_dumper_switch_command())),
        "drive" => state.messenger.send_command(Box::new(state.command_factory.generate_drive_switch_command())),
        _ => return Status::BadRequest,
    }

    Status::Ok
}

/// Runs the drive train at the provided speeds.
#[post("/robot/drive_train/drive/<left>/<right>")]
fn handle_drive(left: f32, right: f32, state: State<ServerState>) -> Status {
    if let Some(command) = state.command_factory.generate_drive_command(left, right) {
        state.messenger.send_command(Box::new(command));
        Status::Ok
    } else {
        Status::BadRequest
    }
}

/// Starts the dumper.
#[post("/robot/dumper/dump")]
fn dump(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_dump_command()));
}

/// Resets the dumper
#[post("/robot/dumper/reset")]
fn reset_dumper(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_reset_dumper_command()));
}

/// Stops the dumper.
#[post("/robot/dumper/stop")]
fn stop_dumper(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_stop_dumper_command()));
}

/// Raises the actuators on the digger.
#[post("/robot/intake/rails/raise")]
fn raise_digger(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_raise_actuators_command()));
}

/// Lower the actuators on the digger.
#[post("/robot/intake/rails/lower")]
fn lower_digger(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_lower_actuators_command()));
}

/// Stops the actuators.
#[post("/robot/intake/rails/stop")]
fn stop_rails(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_stop_actuators_command()));
}

/// Starts the digger.
#[post("/robot/intake/digger/dig")]
fn dig(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_dig_command()));
}

/// Stops the digger.
#[post("/robot/intake/digger/stop")]
fn stop_digger(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_stop_digger_command()));
}

/// Kills the robot. When this command is invoked, all physical motion on the robot ceases.
#[post("/robot/kill")]
fn kill(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_kill_command()));
}

/// Causes the drive train to begin braking.
#[post("/robot/drive_train/brake")]
fn brake(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_brake_command()));
}

/// Revives a dead robot, allowing further motion.
#[post("/robot/revive")]
fn revive(state: State<ServerState>) {
    state.messenger.send_command(Box::new(state.command_factory.generate_revive_command()));
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