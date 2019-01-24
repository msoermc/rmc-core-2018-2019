use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::MotorState;

#[cfg(test)]
mod tests;

pub struct MotorGroup {
    is_inverted: bool,
    motors: Vec<Box<MotorController>>,
    old_speed: f32,
}

impl MotorGroup {
    pub fn new(motors: Vec<Box<MotorController>>) -> Self {
        MotorGroup {
            is_inverted: false,
            motors,
            old_speed: 0.0,
        }
    }

    pub fn set_speed(&mut self, new_speed: f32) -> Result<(), Vec<MotorState>> {
        self.old_speed = new_speed;
        self.run_operation(|motor| motor.set_speed(new_speed))
    }

    pub fn stop(&mut self) -> Result<(), Vec<MotorState>> {
        self.old_speed = 0.0;
        self.run_operation(|motor| motor.stop())
    }

    pub fn invert(&mut self) -> Result<(), Vec<MotorState>> {
        self.is_inverted = !self.is_inverted;
        self.old_speed = -self.old_speed;
        self.run_operation(|motor| motor.invert())
    }

    pub fn is_inverted(&self) -> bool {
        self.is_inverted
    }

    pub fn maintain_last(&mut self) -> Result<(), Vec<MotorState>> {
        self.set_speed(self.old_speed)
    }

    fn run_operation<T: Fn(&mut Box<MotorController>) -> Result<(), MotorState>>(&mut self, operation: T) -> Result<(), Vec<MotorState>> {
        let results: Vec<MotorState> =
            self.motors.iter_mut()
                .map(operation)
                .filter_map(|res| res.err())
                .collect();

        if results.is_empty() {
            Ok(())
        } else {
            Err(results)
        }
    }
}