use super::*;

const TIMEOUT_MILLIS: u64 = 30;

fn get_kill_url() -> String {
    "/robot/kill".to_owned()
}

fn get_revive_url() -> String {
    "/robot/revive".to_owned()
}

#[test]
fn kill() {
    let (state, client) = setup();

    client.post(get_kill_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(false, state.get_life().is_alive());
}

#[test]
fn revive() {
    let (state, client) = setup();

    client.post(get_kill_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    client.post(get_revive_url()).dispatch();
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(true, state.get_life().is_alive());
}