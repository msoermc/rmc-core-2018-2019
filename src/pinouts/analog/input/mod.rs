pub trait AnalogInput: Send {
    fn get_value(&mut self) -> Option<f32>;
}