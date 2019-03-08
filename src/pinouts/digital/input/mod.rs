pub trait DigitalInput: Send {
    fn get_value(&self) -> Option<bool>;
}