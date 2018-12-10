use super::{MotorController, Mo};


pub struct HoverBoardMotor {

}


impl MotorController<HoverBoardError> for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), HoverBoardError> {
        unimplemented!()
    }
    
    fn stop(&mut self) -> Result<(), HoverBoardError> {
        unimplemented!()
    }
    
    fn invert(&mut self) {
        unimplemented!()
    }
    
    fn is_inverted(&self) {
        unimplemented!()
    }
    
    fn enable(&mut self) {
        unimplemented!()
    }
    
    fn disable(&mut self) {
        unimplemented!()
    }
    
    fn is_enabled(&self) {
        unimplemented!()
    }
}


pub struct HoverBoardError {}