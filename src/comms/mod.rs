pub mod external_comms;
pub mod internal_comms;

pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}