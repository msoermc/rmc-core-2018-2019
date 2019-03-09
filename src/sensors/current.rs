use std::sync::Arc;

use crate::framework::Runnable;
use crate::pinouts::analog::input::AnalogInput;
use crate::status::current::GlobalCurrentState;
use rocket::config::Array;
use crate::robot_map::BROWN_CURRENT;

/// Monitors current and updates it's state accordingly.
pub struct CurrentMonitor {
    input: Box<AnalogInput>,
    current: Arc<GlobalCurrentState>,
    old_values: [f32; 3],
}

impl CurrentMonitor {
    pub fn new(input: Box<AnalogInput>, current: Arc<GlobalCurrentState>) -> Self {
        Self {
            input,
            current,
            old_values: oldvalues,
        }
    }
}

impl Runnable for CurrentMonitor {
    fn init(&mut self) {
        self.old_values = [0.0, 0.0, 0.0];
    }

    fn run(&mut self) {
        self.old_values[2] = self.old_values[1];
        self.old_values[1] = self.old_values[0];
        self.old_values[0] = self.input.get_value().unwrap();
        let avg_current : f32 = (self.old_values[0] + self.old_values[1] + self.old_values[2]) / 3;
        self.current.update_current(avg_current);
    }
}