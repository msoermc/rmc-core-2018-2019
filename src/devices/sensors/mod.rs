use std::io;

pub trait DigitalInput {
    fn get_value(&mut self) -> io::Result<bool>;
}