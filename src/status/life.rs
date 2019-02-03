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

    pub fn is_alive(&self) -> bool {
        RobotLifeStatus::Alive as usize == self.status.load(atomic::Ordering::Relaxed)
    }

    pub fn is_dead(&self) -> bool {
        RobotLifeStatus::Dead as usize == self.status.load(atomic::Ordering::Relaxed)
    }

    pub fn kill(&self) {
        self.status.store(RobotLifeStatus::Dead as usize, atomic::Ordering::SeqCst)
    }

    pub fn revive(&self) {
        self.status.store(RobotLifeStatus::Alive as usize, atomic::Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl GlobalLifeStatus {
        fn create_dead() -> Self {
            Self {
                status: Arc::new(atomic::AtomicUsize::new(RobotLifeStatus::Dead as usize))
            }
        }
    }

    #[test]
    fn test_constructor() {
        let status = GlobalLifeStatus::new();
        assert!(status.is_alive());
    }

    #[test]
    fn test_kill() {
        let status = GlobalLifeStatus::new();

        status.kill();

        assert!(status.is_dead());
    }

    #[test]
    fn test_revive() {
        let status = GlobalLifeStatus::create_dead();

        status.kill();
        status.revive();

        assert!(status.is_alive());
    }
}