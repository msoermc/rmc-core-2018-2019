use super::{DriveTrain, DriveTrainError};

pub trait TankError: DriveTrainError {

}

pub trait TankDriveTrain<E: DriveTrainError>: DriveTrain<E> {
    fn drive(&mut self, left_speed: f32, right_speed: f32);
}