pub mod hover_board;

pub trait MotorController: Send {
    /// Sets the current speed of the motor controller.
    /// The speed should be a floating point number between -1 and 1.
    /// A negative speed indicates that the direction is reversed.
    fn set_speed(&mut self, new_speed: f32);


    /// Sets the current speed of the motor controller to zero.
    fn stop(&mut self);


    /// Inverts the directionality of the motor controller.
    fn invert(&mut self);


    /// Returns true if the motor controller is inverted and false otherwise.
    fn is_inverted(&self) -> bool;
}