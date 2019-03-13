use std::sync::atomic::Ordering;
use std::thread::spawn;

use super::*;

#[test]
fn controller_cycles_per_second() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_bench();
    let robot = builder.generate().assemble();
    spawn(|| robot.launch());

    sleep(Duration::from_secs(2));
    let rate = state.get_cycles_per_second().load(Ordering::SeqCst);
    assert!(rate >= 1_000, "val is {}", rate);
}