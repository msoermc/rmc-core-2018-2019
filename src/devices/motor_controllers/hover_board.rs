use super::{MotorController};

impl MotorController for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorController> {
        unimplemented!()
    }
    
    fn stop(&mut self) -> Result<(), MotorController> {
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
