use super::*;

pub struct PrintMotor {
    name: String,
    inverted: bool,
    last: f32
}

impl MotorController for PrintMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure> {
        if self.last != new_speed {
            println!("{}: -> {}", self.name, new_speed);
            self.last = new_speed;
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        println!("{}: STOP", self.name);
        Ok(())
    }

    fn invert(&mut self) -> Result<(), MotorFailure> {
        println!("{}: INVERT", self.name);
        self.inverted = !self.inverted;
        Ok(())
    }

    fn is_inverted(&self) -> Result<bool, MotorFailure> {
        Ok(self.inverted)
    }
}

impl PrintMotor {
    pub fn new(name: &str) -> PrintMotor {
        PrintMotor {
            name: name.to_string(),
            inverted: false,
            last: -10.0
        }
    }
}