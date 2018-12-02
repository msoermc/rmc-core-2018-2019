use super::{DriveTrainLogPayload, DriveTrain};

pub trait TankDriveTrainLogPayload: DriveTrainLogPayload {

}

pub trait TankDriveTrain<P: TankDriveTrainLogPayload>: DriveTrain<P> {
    fn drive(&mut self, left_speed: f32, right_speed: f32);
}