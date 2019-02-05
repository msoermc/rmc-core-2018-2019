use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub const LADDER_RUNNING: usize = 1;
pub const LADDER_STOPPED: usize = 0;

pub struct GlobalLadderState {
    action: AtomicUsize,
}

impl GlobalLadderState {
    pub fn new() -> Self {
        Self {
            action: AtomicUsize::new(LADDER_STOPPED)
        }
    }

    pub fn get_current_state(&self) -> LadderStateInstance {
        LadderStateInstance::new(self.action.load(Ordering::Relaxed))
    }

    pub fn set_action(&self, action: usize) {
        self.action.store(action, Ordering::Relaxed);
    }

    pub fn get_action(&self) -> usize {
        self.action.load(Ordering::Relaxed)
    }
}

#[derive(Serialize)]
pub struct LadderStateInstance {
    action: usize
}

impl LadderStateInstance {
    fn new(action: usize) -> Self {
        Self {
            action,
        }
    }

    pub fn get_action(&self) -> usize {
        self.action
    }
}