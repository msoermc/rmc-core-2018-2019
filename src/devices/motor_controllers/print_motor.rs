use super::*;

pub struct PrintMotor {
    name: String,
    inverted: bool,
}

impl MotorController for PrintMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), LogData> {
        unimplemented!()
    }

    fn stop(&mut self) -> Result<(), LogData> {
        unimplemented!()
    }

    fn invert(&mut self) -> Result<(), LogData> {
        unimplemented!()
    }

    fn is_inverted(&self) -> Result<bool, LogData> {
        unimplemented!()
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