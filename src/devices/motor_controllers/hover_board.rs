use super::{
    MotorController,
};

pub struct HoverBoardMotor {}

impl MotorController<HoverBoardError> for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), HoverBoardError> {
        unimplemented!()
    }

    fn stop(&mut self) -> Result<(), HoverBoardError> {
        unimplemented!()
    }

    fn invert(&mut self) -> Result<(), HoverBoardError>{
        unimplemented!()
    }

    fn is_inverted(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HoverBoardError {

}