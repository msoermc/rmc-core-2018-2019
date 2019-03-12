use std::sync::Arc;

use atomic::{Atomic, Ordering};

use crate::robot_map::{BROWN_CURRENT, CRITICAL_CURRENT};
use crate::status::current::CurrentUsageLevel::{Brownout, Critical, Normal};

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Debug)]
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
        self.current.store(current, Ordering::Relaxed);
        if current >= CRITICAL_CURRENT {
            self.level.store(Critical, Ordering::Relaxed);
        } else if current >= BROWN_CURRENT {
            self.level.store(Brownout, Ordering::Relaxed);
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
}

#[cfg(test)]
mod tests {
    use crate::robot_map::NORMAL_CURRENT;

    use super::*;

    #[test]
    fn initial() {
        let state = GlobalCurrentState::new();
        assert_eq!(CurrentUsageLevel::Normal, state.get_level());
        assert_eq!(0.0, state.get_current());
        assert!(state.is_normal());
    }

    #[test]
    fn update_normal() {
        let mut state = GlobalCurrentState::new();

        state.update_current(CRITICAL_CURRENT);
        state.update_current(NORMAL_CURRENT);

        assert_eq!(CurrentUsageLevel::Normal, state.get_level());
        assert_eq!(NORMAL_CURRENT, state.get_current());
        assert!(state.is_normal());
    }

    #[test]
    fn update_brownout() {
        let mut state = GlobalCurrentState::new();

        state.update_current(BROWN_CURRENT);

        assert_eq!(CurrentUsageLevel::Brownout, state.get_level());
        assert_eq!(BROWN_CURRENT, state.get_current());
        assert_eq!(false, state.is_normal());
    }

    #[test]
    fn update_critical() {
        let mut state = GlobalCurrentState::new();

        state.update_current(CRITICAL_CURRENT);

        assert_eq!(CurrentUsageLevel::Critical, state.get_level());
        assert_eq!(CRITICAL_CURRENT, state.get_current());
        assert_eq!(false, state.is_normal());
    }
}