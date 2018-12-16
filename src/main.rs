use crate::test_programs::test_logging;
use crate::test_programs::test_comms;

pub mod framework;
pub mod subsystems;
pub mod devices;
pub mod test_programs;
pub mod comms;

fn main() {
    test_comms::run_test();
}
