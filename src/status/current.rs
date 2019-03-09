use std::sync::Arc;

use atomic::{Atomic, Ordering};
use crate::robot_map::{BROWN_CURRENT, CRITICAL_CURRENT};
use crate::status::current::CurrentUsageLevel::{Brownout, Critical, Normal};

#[derive(Copy, Clone, Eq, PartialEq, Serialize)]
pub enum CurrentUsageLevel {
    Normal = 0,
    Brownout = 1,
    Critical = 2,
}

#[derive(Clone)]
pub struct GlobalCurrentState {
    current: Arc<Atomic<f32>>,
    level: Arc<Atomic<CurrentUsageLevel>>,
}

impl GlobalCurrentState {
    pub fn new() -> Self {
        Self {
            current: Arc::new(Atomic::new(0.0)),
            level: Arc::new(Atomic::new(CurrentUsageLevel::Normal)),
        }
    }

    pub fn get_current(&self) -> f32 {
        self.current.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_level(&self) -> CurrentUsageLevel {
        self.level.load(Ordering::Relaxed)
    }

    pub fn is_normal(&self) -> bool {
        CurrentUsageLevel::Normal == self.get_level()
    }

    pub fn update_current(&mut self, current: f32) {
        if current > BROWN_CURRENT {
            self.level.store(Brownout, Ordering::Relaxed);
        } else if current > CRITICAL_CURRENT {
            self.level.store(Critical, Ordering::Relaxed);
        } else {
            self.level.store(Normal, Ordering::Relaxed);
        }
    }

    pub fn get_json(&self) -> CurrentStateJson {
        CurrentStateJson::new(self.current.load(Ordering::Relaxed),
                              self.level.load(Ordering::Relaxed))
    }
}

#[derive(Serialize)]
pub struct CurrentStateJson {
    current: f32,
    level: CurrentUsageLevel,
}

impl CurrentStateJson {
    fn new(current: f32, level: CurrentUsageLevel) -> Self {
        Self {
            current,
            level,
        }
    }

    pub fn get_current(&self) -> f32 {
        self.current
    }

    pub fn get_level(&self) -> CurrentUsageLevel {
        self.level
    }
}