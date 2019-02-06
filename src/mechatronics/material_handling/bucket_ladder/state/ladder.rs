use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub struct GlobalLadderState {}

impl GlobalLadderState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_current_state(&self) -> LadderStateInstance {
        LadderStateInstance::new()
    }
}

#[derive(Serialize)]
pub struct LadderStateInstance {}

impl LadderStateInstance {
    fn new() -> Self {
        Self {}
    }
}