pub trait DigitalInput {
    fn get_value(&self) -> Option<bool>;
}