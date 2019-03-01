use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::motor_controllers::GlobalMotorState;
use crate::motor_controllers::MotorStateInstance;

#[derive(Default)]
pub struct GlobalActuatorState {
    upper: Arc<AtomicBool>,
    lower: Arc<AtomicBool>,
    motor: Arc<GlobalMotorState>,
}

impl GlobalActuatorState {
    pub fn new() -> Self {
        GlobalActuatorState {
            upper: Arc::new(AtomicBool::new(false)),
            lower: Arc::new(AtomicBool::new(false)),
            motor: Arc::new(GlobalMotorState::new()),
        }
    }

    pub fn get_current_state(&self) -> ActuatorStateInstance {
        ActuatorStateInstance::new(
            self.upper.load(Ordering::Relaxed),
            self.lower.load(Ordering::Relaxed),
            self.motor.get_current_state(),
        )
    }

    pub fn set_upper(&self, upper: bool) {
        self.upper.store(upper, Ordering::Relaxed);
    }

    pub fn set_lower(&self, lower: bool) {
        self.lower.store(lower, Ordering::Relaxed);
    }

    pub fn get_upper(&self) -> Arc<AtomicBool> {
        self.upper.clone()
    }

    pub fn get_lower(&self) -> Arc<AtomicBool> {
        self.lower.clone()
    }

    pub fn get_motor(&self) -> Arc<GlobalMotorState> {
        self.motor.clone()
    }
}

#[derive(Serialize)]
pub struct ActuatorStateInstance {
    upper: bool,
    lower: bool,
    motor: MotorStateInstance,
}

impl ActuatorStateInstance {
    fn new(upper: bool, lower: bool, motor: MotorStateInstance) -> Self {
        ActuatorStateInstance {
            upper,
            lower,
            motor,
        }
    }

    pub fn get_upper(&self) -> bool {
        self.upper
    }

    pub fn get_lower(&self) -> bool {
        self.lower
    }

    pub fn get_motor(&self) -> &MotorStateInstance {
        &self.motor
    }
}