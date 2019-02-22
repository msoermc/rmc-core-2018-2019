pub trait DigitalOutput: Send {
    fn set_value(&mut self, val: bool) -> Result<(), String>;
}

