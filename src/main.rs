use crate::run_modes::demo_mode::run_demo_mode;

pub mod framework;
pub mod devices;
pub mod run_modes;
pub mod comms;
pub mod drive_train;
pub mod logging;

fn main() {
    run_demo_mode();
}
