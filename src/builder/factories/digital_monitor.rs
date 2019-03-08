use std::rc::Rc;
use std::sync::Arc;

use crate::builder::factories::SubsystemFactory;
use crate::framework::{CompositeRunnable, Runnable};
use crate::pinouts::factories::IoFactory;
use crate::status::robot_state::GlobalRobotState;
use crate::pinouts::digital::TestPin;
use crate::pinouts::digital::input::DigitalInput;

pub struct DigitalMonitorFactory {
    state: Arc<GlobalRobotState>,
    input: Box<DigitalInput>,
}

pub struct NullDigitalMonitorFactory {
}

impl DigitalMonitorFactory {
    pub fn new(state: Arc<GlobalRobotState>, input: Box<DigitalInput>) -> Self {
        Self {
            state,
            input,
        }
    }
}

impl ToString for DigitalMonitorFactory {
    fn to_string(&self) -> String {
        "production monitor".to_owned()
    }
}

impl SubsystemFactory<Box<Runnable>> for DigitalMonitorFactory {
    fn produce(&self) -> Box<Runnable> {
        unimplemented!()
    }
}