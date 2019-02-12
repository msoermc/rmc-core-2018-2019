use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::Ordering;

/// Used for benchmarking the controller
pub struct ControllerBench {
    controller_cycle_counter: Arc<AtomicUsize>,
    average: Arc<AtomicUsize>,
    total: usize,
    total_secs: usize,
}

impl ControllerBench {
    /// Constructs
    pub fn new(controller_cycle_counter: Arc<AtomicUsize>, average: Arc<AtomicUsize>) -> Self {
        Self {
            controller_cycle_counter,
            average,
            total: 0,
            total_secs: 0,
        }
    }

    /// Launches the benchmarking thread, taking over this thread.
    pub fn launch(mut self) {
        loop {
            sleep(Duration::from_secs(1));
            let new_count = self.controller_cycle_counter.load(Ordering::Relaxed);
            self.total += new_count;
            self.total_secs += 1;
            self.average.store(self.total / self.total_secs, Ordering::Relaxed);
        }
    }
}