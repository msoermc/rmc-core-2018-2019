use super::*;

pub mod hover_board;

pub trait MotorController<E>: Device {
    /// Sets the current speed of the motor controller.
    /// The speed should be a floating point number between -1 and 1.
    /// A negative speed indicates that the direction is reversed.
    fn set_speed(&mut self, new_speed: f32) -> Result<(), E>;
    
    
    /// Sets the current speed of the motor controller to zero.
    fn stop(&mut self) -> Result<(), E>;
    
    
    /// Inverts the directionality of the motor controller.
    fn invert(&mut self);
    
    
    /// Returns true if the motor controller is inverted and false otherwise.
    fn is_inverted(&self);
    
    
    /// Enables the motor controller.
    /// While enabled, motor controllers operate as normal.
    /// Motor controllers should be enabled by default.
    fn enable(&mut self);
    
    
    /// Disables the motor controller.
    /// While disabled, motor controllers will not run their motors.
    fn disable(&mut self);
    
    
    /// Returns true if the motor controller is enabled and false otherwise.
    fn is_enabled(&self);
}