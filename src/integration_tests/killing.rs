use crate::comms::RobotLifeRestId;
use rocket::http::ContentType;
use super::*;
use rocket::Response;
use rocket::local::LocalResponse;

const TIMEOUT_MILLIS: u64 = 30;

pub fn send_life(client: &Client, life: RobotLifeRestId) -> LocalResponse {
    client.post("/robot").header(ContentType::JSON).body(
        match life {
            RobotLifeRestId::Alive => r#"{ "life" : "Alive" }"#,
            RobotLifeRestId::Dead =>  r#"{ "life" : "Dead" }"#,
        }
    ).dispatch()
}

#[test]
fn kill() {
    let (state, client) = setup();

    let response = send_life(&client, RobotLifeRestId::Dead);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));
    assert_eq!(false, state.get_life().is_alive());
}

#[test]
fn revive() {
    let (state, client) = setup();

    let response = send_life(&client, RobotLifeRestId::Dead);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    let response = send_life(&client, RobotLifeRestId::Alive);
    sleep(Duration::from_millis(TIMEOUT_MILLIS));

    assert_eq!(true, state.get_life().is_alive());
}