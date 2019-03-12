use super::*;

const TIMEOUT_MILLIS: u64 = 30;

fn get_enable_dumper_url() -> String {
    "/robot/modes/dump".to_owned()
}

fn get_dump_url() -> String {
    "/robot/dumper/dump".to_owned()
}

fn get_reset_url() -> String {
    "/robot/dumper/reset".to_owned()
}

fn get_stop_url() -> String {
    "/robot/dumper/stop".to_owned()
}

#[test]
fn dump() {
    let (state, client) = setup();
    client.post(get_enable_dumper_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_dump_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(DUMPING_RATE, state.get_current_state().get_dumper().get_motor().get_speed());
    assert_eq!(DUMPING_RATE, state.get_dumper().get_current_state().get_motor().get_speed());
}

#[test]
fn reset() {
    let (state, client) = setup();
    client.post(get_enable_dumper_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_reset_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(DUMPER_RESET_RATE, state.get_current_state().get_dumper().get_motor().get_speed());
    assert_eq!(DUMPER_RESET_RATE, state.get_dumper().get_current_state().get_motor().get_speed());
}

#[test]
fn stop() {
    let (state, client) = setup();
    client.post(get_enable_dumper_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_dump_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_stop_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(0.0, state.get_current_state().get_dumper().get_motor().get_speed());
    assert_eq!(0.0, state.get_dumper().get_current_state().get_motor().get_speed());
}