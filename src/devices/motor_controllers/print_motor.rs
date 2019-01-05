use super::*;

pub struct PrintMotor {
    name: String,
    inverted: bool,
}

impl MotorController for PrintMotor {
    fn set_speed(&mut self, new_speed: f32) {
        println!("{}: Set speed to {}!", self.name, new_speed);
    }

    fn stop(&mut self) {
        println!("{}: Stop!", self.name);
    }

    fn invert(&mut self) {
        self.inverted = !self.inverted;
        println!("{}: Invert!", self.name);
    }

    fn is_inverted(&self) -> bool {
        self.inverted
    }
}

impl PrintMotor {
    pub fn new(name: &str) -> PrintMotor {
        PrintMotor {
            name: name.to_string(),
            inverted: false,
        }
    }
}