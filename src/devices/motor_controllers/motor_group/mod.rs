use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::MotorState;

#[cfg(test)]
mod tests;

pub struct MotorGroup {
    motors: Vec<Box<MotorController>>,
    old_speed: f32,
}

impl MotorGroup {
    pub fn new(motors: Vec<Box<MotorController>>) -> Self {
        MotorGroup {
            motors,
            old_speed: 0.0,
        }
    }

    pub fn set_speed(&mut self, new_speed: f32) {
        self.old_speed = new_speed;
        self.run_operation(|motor| motor.set_speed(new_speed))
    }

    pub fn stop(&mut self) {
        self.old_speed = 0.0;
        self.run_operation(|motor| motor.stop())
    }

    pub fn invert(&mut self) {
        self.old_speed = -self.old_speed;
        self.run_operation(|motor| motor.invert())
    }

    pub fn maintain_last(&mut self) {
        self.set_speed(self.old_speed)
    }

    pub fn get_states(&self) -> Vec<MotorState> {
        self.motors.iter().map(|motor| motor.get_motor_state()).collect()
    }

    fn run_operation<T: Fn(&mut Box<MotorController>)>(&mut self, operation: T) {
        self.motors.iter_mut().for_each(operation);
    }
}