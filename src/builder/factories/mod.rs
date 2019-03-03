pub mod drive;
pub mod dumper;
pub mod intake;

pub trait SubsystemFactory<T>: ToString {
    fn produce(&self) -> T;
}