pub mod drive;
pub mod dumper;
pub mod intake;
pub mod digital_monitor;
pub mod arduino;

pub trait SubsystemFactory<T>: ToString {
    fn produce(self: Box<Self>) -> T;
}