pub mod drive;
pub mod dumper;
pub mod intake;
pub mod digital_monitor;

pub trait SubsystemFactory<T>: ToString {
    fn produce(&self) -> T;
}