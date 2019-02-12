use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use crate::pinouts::AnalogInput;
use atomic::Atomic;

/// Launches a power monitor that will take over this thread.
pub fn launch_power_monitor<T: AnalogInput>(current_var: Arc<Atomic<f32>>, input: T) {

}