use crate::framework::subsystem::*;

mod tank;

pub trait DriveTrainLogPayload {

}

pub trait DriveTrain<P: DriveTrainLogPayload>: Subsystem<P> {
    fn brake(&mut self) -> P;
}