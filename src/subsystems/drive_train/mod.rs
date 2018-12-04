use crate::framework::{Subsystem, RobotError};

pub mod tank;

pub trait DriveTrainError: RobotError {

}

pub trait DriveTrain<E: DriveTrainError>: Subsystem<E> {
    fn brake(&mut self);
}