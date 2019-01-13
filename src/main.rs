use crate::run_modes::demo_mode::run_demo_mode;
use crate::run_modes::run_drive_train::run_drive_train;

pub mod framework;
pub mod devices;
pub mod run_modes;
pub mod comms;
pub mod robot_control;
pub mod logging;
pub mod robot_map;

fn main() {
    run_drive_train();
}
