use crate::test_programs::test_logging;

pub mod framework;
pub mod subsystems;
pub mod devices;
pub mod test_programs;
pub mod drive_train;

fn main() {
    test_logging::run_test();
}
