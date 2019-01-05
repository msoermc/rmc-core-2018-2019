use super::MotorController;

pub struct HoverBoardMotor {}

impl MotorController for HoverBoardMotor {
    fn set_speed(&mut self, new_speed: f32) {
        unimplemented!()
    }

    fn stop(&mut self) {
        unimplemented!()
    }

    fn invert(&mut self) {
        unimplemented!()
    }

    fn is_inverted(&self) -> bool {
        unimplemented!()
    }
}