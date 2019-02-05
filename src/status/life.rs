use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct GlobalLifeState {
    life: AtomicBool
}

impl GlobalLifeState {
    pub fn new() -> Self {
        Self {
            life: AtomicBool::new(true)
        }
    }

    pub fn is_alive(&self) -> bool {
        self.life.load(atomic::Ordering::Relaxed)
    }

    pub fn kill(&self) {
        self.life.store(false, atomic::Ordering::SeqCst)
    }

    pub fn revive(&self) {
        self.life.store(true, atomic::Ordering::SeqCst)
    }

    pub fn get_current_state(&self) -> LifeStateInstance {
        LifeStateInstance::new(self.life.load(Ordering::Relaxed))
    }
}

#[derive(Serialize)]
pub struct LifeStateInstance {
    life: bool
}

impl LifeStateInstance {
    fn new(life: bool) -> Self {
        Self {
            life
        }
    }

    pub fn get_life(&self) -> bool {
        self.life
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl GlobalLifeState {
        fn create_dead() -> Self {
            Self {
                life: Arc::new(AtomicBool::new(false))
            }
        }
    }

    #[test]
    fn test_constructor() {
        let status = GlobalLifeState::new();
        assert!(status.is_alive());
    }

    #[test]
    fn test_kill() {
        let status = GlobalLifeState::new();

        status.kill();

        assert!(!status.is_alive());
    }

    #[test]
    fn test_revive() {
        let status = GlobalLifeState::create_dead();

        status.kill();
        status.revive();

        assert!(status.is_alive());
    }
}