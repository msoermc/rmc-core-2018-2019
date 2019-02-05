use std::sync::atomic::AtomicUsize;

pub const LADDER_RUNNING: u8 = 1;
pub const LADDER_STOPPED: u8 = 0;

pub struct GlobalLadderState {
    action: AtomicUsize,
}

#[derive(Serialize)]
pub struct LadderStateInstance {
    action: usize
}