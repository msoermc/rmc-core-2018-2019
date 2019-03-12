use super::*;

const TIMEOUT_MILLIS: u64 = 30;

fn get_enable_url() -> String {
    "/robot/modes/dig".to_owned()
}

fn get_digging_url() -> String {
    "/robot/intake/digger/dig".to_owned()
}

#[test]
fn dig() {
    let (state, client) = setup();
    client.post(get_enable_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_digging_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(DIGGING_RATE, state.get_current_state().get_intake().get_digger().get_speed());
    assert_eq!(DIGGING_RATE, state.get_intake().get_current_state().get_digger().get_speed());
}