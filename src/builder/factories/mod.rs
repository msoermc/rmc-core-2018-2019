pub mod drive;
pub mod dumper;
pub mod intake;
pub mod arduino;

pub trait SubsystemFactory<T>: ToString {
    fn produce(self: Box<Self>) -> T;
}