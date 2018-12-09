use super::*;


pub trait MotorController<E>: Device {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), E>;
    
    fn stop(&mut self);
    
    fn invert(&mut self);
    
    fn is_inverted(&self);
    
    fn enable(&mut self);
    
    fn disable(&mut self);
    
    fn is_enabled(&self);
}