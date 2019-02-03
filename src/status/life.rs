use std::sync::Arc;
use std::sync::atomic;

/// Represents the current status of the robot.
/// Many subsystems will check this before determining if it is safe to perform an operation.
#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
pub enum RobotLifeStatus {
    /// Indicates that the robot is in a normal operating state.
    Alive = 0,

    /// Indicates that the robot has been disabled by the operators and that it is not
    /// safe to perform many operations.
    Dead = 1,
}

#[derive(Clone)]
pub struct GlobalLifeStatus {
    status: Arc<atomic::AtomicUsize>
}

impl GlobalLifeStatus {
    pub fn new() -> Self {
        Self {
            status: Arc::new(atomic::AtomicUsize::new(RobotLifeStatus::Alive as usize))
        }
    }

    pub fn get_status(&self) -> RobotLifeStatus {
        num::FromPrimitive::from_usize(self.status.load(atomic::Ordering::Relaxed)).unwrap()
    }

    pub fn is_alive(&self) -> bool {
        self.status.load(atomic::Ordering::Relaxed) == RobotLifeStatus::Alive as usize
    }

    pub fn kill(&self) {
        self.status.store(RobotLifeStatus::Dead as usize, atomic::Ordering::SeqCst)
    }

    pub fn revive(&self) {
        self.status.store(RobotLifeStatus::Alive as usize, atomic::Ordering::SeqCst)
    }
}