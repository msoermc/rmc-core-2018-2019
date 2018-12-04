use crate::framework::Subsystem;

pub mod tank;

pub trait DriveTrainLogPayload {

}

pub trait DriveTrain: Subsystem {
    fn brake(&mut self);
}