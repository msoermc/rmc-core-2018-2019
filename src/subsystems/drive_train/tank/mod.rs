use super::DriveTrain;

pub trait TankDriveTrain: DriveTrain {
    fn drive(&mut self, left_speed: f32, right_speed: f32);
}