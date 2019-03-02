pub mod drive;
pub mod dumper;
pub mod intake;

pub trait SubsystemFactory<T> {
    fn produce(&self) -> T;
}