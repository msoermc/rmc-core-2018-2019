use super::*;
use std::sync::atomic::Ordering;

#[test]
fn controller_cycles_per_second() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    builder.with_bench();
    let robot = builder.build();
    let _client = robot.launch_tester();

    sleep(Duration::from_secs(2));
    let rate = state.get_cycles_per_second().load(Ordering::SeqCst);
    assert!(rate >= 1_000, "val is {}", rate);
}