use super::*;

use std::io::Result;

pub mod hover_board;

pub trait MotorController: Device {
    /// Sets the current speed of the motor controller.
    /// The speed should be a floating point number between -1 and 1.
    /// A negative speed indicates that the direction is reversed.
    fn set_speed(&mut self, new_speed: f32) -> Result<()>;
    
    
    /// Sets the current speed of the motor controller to zero.
    fn stop(&mut self) -> Result<()>;
    
    
    /// Inverts the directionality of the motor controller.
    fn invert(&mut self);
    
    
    /// Returns true if the motor controller is inverted and false otherwise.
    fn is_inverted(&self) -> bool;
    
    
    /// Enables the motor controller.
    /// While enabled, motor controllers operate as normal.
    /// Motor controllers should be enabled by default.
    fn enable(&mut self);
    
    
    /// Disables the motor controller.
    /// While disabled, motor controllers will not run their motors.
    fn disable(&mut self);
    
    
    /// Returns true if the motor controller is enabled and false otherwise.
    fn is_enabled(&self) -> bool;
    
    fn run_at_previous_speed(&mut self);
}