use super::MotorController;
use crate::devices::Device;

use std::io::Result;

pub struct HoverBoardMotor {

}

impl MotorController for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<()> {
        unimplemented!()
    }
    
    fn stop(&mut self) -> Result<()> {
        unimplemented!()
    }
    
    fn invert(&mut self) {
        unimplemented!()
    }
    
    fn is_inverted(&self) -> bool {
        unimplemented!()
    }
}

impl Device for HoverBoardMotor {

}