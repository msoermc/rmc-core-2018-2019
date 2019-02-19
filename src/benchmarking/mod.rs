use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;

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
            let new_count = self.controller_cycle_counter.load(Ordering::SeqCst);
            self.controller_cycle_counter.store(0, Ordering::SeqCst);
            self.total += new_count;
            self.total_secs += 1;
            let average = self.total / self.total_secs;
            self.average.store(average, Ordering::SeqCst);
            info!("Current cycle rate: {}", new_count);
            info!("Total cycle count: {}", self.total);
            info!("Average cycle rate: {}", average);
        }
    }
}