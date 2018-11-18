pub enum RobotError<R: RecoverableError, N: NonRecoverableError> {
    Recoverable(R), NonRecoverable(N)
}

pub trait RecoverableError {

}

pub trait NonRecoverableError {

}

pub trait InitError {

}

pub trait OKStatus {

}