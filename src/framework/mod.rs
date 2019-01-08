pub mod interfaces;

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
        }
    }
}