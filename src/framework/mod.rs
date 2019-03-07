use std::thread;

/// The runnable trait represents a process which should initialize itself and run repeatedly.
pub trait Runnable {
    /// Initializes the Runnable, returning a result object indicating whether the action was
    /// successful.
    fn init(&mut self);

    /// Runs a single loop of the Runnable. This function will be called repeatedly by the
    /// framework.
    ///
    /// This function should return false when this Runnable is stopping.
    fn run(&mut self);

    /// Starts the Runnable in the current thread. The Runnable will take over the current thread
    /// when this method is invoked.
    fn start(&mut self) {
        self.init();
        loop {
            self.run();
            thread::yield_now();
        }
    }
}

pub struct CompositeRunnable {
    children: Vec<Box<Runnable>>
}

impl Runnable for CompositeRunnable {
    fn init(&mut self) {
        for child in &mut self.children {
            child.init();
        }
    }

    fn run(&mut self) {
        for child in &mut self.children {
            child.run();
        }
    }
}

impl CompositeRunnable {
    pub fn new() -> Self {
        Self {
            children: Vec::new()
        }
    }

    pub fn add_runnable(&mut self, runnable: Box<Runnable>) {
        self.children.push(runnable)
    }
}