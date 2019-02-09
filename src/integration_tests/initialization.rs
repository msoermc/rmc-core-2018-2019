use super::*;

#[test]
fn life() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let _client = robot.launch_tester();

    assert_eq!(true, state.get_life().get_current_state().get_life());
}

#[test]
fn enabling() {
    let mut builder = RobotBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.build();
    let _client = robot.launch_tester();

    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}
