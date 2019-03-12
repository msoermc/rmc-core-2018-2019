use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use rocket::response::NamedFile;
use rocket::Rocket;
use rocket::State;
use rocket_contrib::json::Json;

use crate::mechatronics::commands::RobotCommandFactory;
use crate::mechatronics::RobotMessenger;
use crate::status::robot_state::GlobalRobotState;
use crate::status::robot_state::RobotStateInstance;

use rocket::http::Status;

#[cfg(test)]
mod tests;

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
                              put_robot,
                              put_drive,
                              ])
}

#[derive(Deserialize, Serialize)]
pub enum RobotMode {
    Digging,
    Driving,
    Dumping,
}

#[derive(Serialize, Deserialize)]
pub enum RobotLifeRestId {
    Alive,
    Dead,
}

#[derive(Deserialize, Serialize)]
struct RobotPostRequest {
    mode: Option<RobotMode>,
    life: Option<RobotLifeRestId>,
}

#[derive(Serialize, Deserialize)]
pub enum DriveTrainAction {
    Drive { left: f32, right: f32 },
    Brake
}

#[put("/robot", format = "application/json", data = "<robot>")]
fn put_robot(robot: Json<RobotPostRequest>, messenger: State<RobotMessenger>, factory: State<RobotCommandFactory>) {
    let RobotPostRequest { mode: mode_opt, life: life_opt } = robot.into_inner();

    if let Some(life) = life_opt {
        match life {
            RobotLifeRestId::Alive => messenger.send_command(Box::new(factory.generate_revive_command())),
            RobotLifeRestId::Dead => messenger.send_command(Box::new(factory.generate_kill_command())),
        }
    };

    if let Some(mode) = mode_opt {
        match mode {
            RobotMode::Digging => messenger.send_command(Box::new(factory.generate_intake_switch_command())),
            RobotMode::Driving => messenger.send_command(Box::new(factory.generate_drive_switch_command())),
            RobotMode::Dumping => messenger.send_command(Box::new(factory.generate_dumper_switch_command())),
        }
    };
}

#[put("/robot/drive", format = "application/json", data = "<action>")]
fn put_drive(action: Json<DriveTrainAction>, messenger: State<RobotMessenger>, factory: State<RobotCommandFactory>) -> Status {
    match action.into_inner() {
        DriveTrainAction::Drive { left, right } => {
            if let Some(command) = factory.generate_drive_command(left, right) {
                messenger.send_command(Box::new(command));
                Status::Ok
            } else {
                Status::BadRequest
            }
        },
        DriveTrainAction::Brake => {
            messenger.send_command(Box::new(factory.generate_brake_command()));
            Status::Ok
        },
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