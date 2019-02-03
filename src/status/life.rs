use std::sync::Arc;
use std::sync::atomic;

const ALIVE: usize = 0;
const DEAD: usize = 1;

#[derive(Clone)]
pub struct GlobalLifeStatus {
    status: Arc<atomic::AtomicUsize>
}

impl GlobalLifeStatus {
    pub fn new() -> Self {
        Self {
            status: Arc::new(atomic::AtomicUsize::new(ALIVE))
        }
    }

    pub fn is_alive(&self) -> bool {
        ALIVE == self.status.load(atomic::Ordering::Relaxed)
    }

    pub fn is_dead(&self) -> bool {
        DEAD == self.status.load(atomic::Ordering::Relaxed)
    }

    pub fn kill(&self) {
        self.status.store(DEAD, atomic::Ordering::SeqCst)
    }

    pub fn revive(&self) {
        self.status.store(ALIVE, atomic::Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl GlobalLifeStatus {
        fn create_dead() -> Self {
            Self {
                status: Arc::new(atomic::AtomicUsize::new(DEAD))
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