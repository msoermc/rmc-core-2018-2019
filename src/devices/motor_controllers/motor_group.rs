use crate::devices::motor_controllers::MotorController;
use crate::devices::motor_controllers::MotorFailure;

pub struct MotorGroup {
    is_inverted: bool,
    motors: Vec<Box<MotorController>>,
}

impl MotorGroup {
    pub fn new(motors: Vec<Box<MotorController>>) -> Self {
        MotorGroup {
            is_inverted: false,
            motors,
        }
    }

    pub fn set_speed(&mut self, new_speed: f32) -> Result<(), Vec<MotorFailure>> {
        self.run_operation(|mut motor| motor.set_speed(new_speed))
    }

    pub fn stop(&mut self) -> Result<(), Vec<MotorFailure>> {
        self.set_speed(0.0)
    }

    pub fn invert(&mut self) -> Result<(), Vec<MotorFailure>> {
        unimplemented!()
    }

    pub fn is_inverted(&self) -> Result<bool, Vec<MotorFailure>> {
        unimplemented!()
    }

    pub fn maintain_last(&mut self) -> Result<(), Vec<MotorFailure>> {
        unimplemented!()
    }

    fn run_operation<T: Fn(&mut Box<MotorController>) -> Result<(), MotorFailure>>(&mut self, operation: T) -> Result<(), Vec<MotorFailure>> {
        let results: Vec<MotorFailure> = self.motors.iter_mut().map(operation).filter_map(|res| res.err()).collect();

        if results.is_empty() {
            Ok(())
        } else {
            Err(results)
        }
    }
}



