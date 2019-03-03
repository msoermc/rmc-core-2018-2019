use super::*;

#[test]
fn life() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let _client = robot.launch_tester();

    assert_eq!(true, state.get_current_state().get_life().get_life());
}

#[test]
fn enabling() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let _client = robot.launch_tester();

    assert_eq!(false, state.get_drive().get_current_state().get_enabled());
    assert_eq!(false, state.get_dumper().get_current_state().get_enabled());
    assert_eq!(false, state.get_intake().get_current_state().get_enabled());
}

#[test]
fn motors() {
    let mut builder = RobotAssemblyBuilder::new();
    let state = builder.get_state();
    builder.with_test();
    let robot = builder.generate().assemble();
    let _client = robot.launch_tester();

    assert_eq!(0.0, state.get_intake().get_current_state().get_left_actuator().get_motor().get_speed());
    assert_eq!(0.0, state.get_intake().get_current_state().get_right_actuator().get_motor().get_speed());
    assert_eq!(0.0, state.get_intake().get_ladder().get_current_state().get_motor().get_speed());

    assert_eq!(0.0, state.get_drive().get_current_state().get_left().get_speed());
    assert_eq!(0.0, state.get_drive().get_current_state().get_right().get_speed());

    assert_eq!(0.0, state.get_dumper().get_motor().get_current_state().get_speed());
}