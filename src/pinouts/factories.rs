use crate::pinouts::analog::output::AnalogOutput;
use crate::pinouts::analog::output::pwm::LibBeagleBonePwm;
use crate::pinouts::analog::output::PwmOutput;
use crate::pinouts::digital::input::DigitalInput;
use crate::pinouts::digital::libbeaglebone::GpioPinout;
use crate::pinouts::digital::output::DigitalOutput;

pub struct IoFactory {}

impl IoFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_pwm(&self, chip: u8, num: u8) -> Box<PwmOutput> {
        Box::new(LibBeagleBonePwm::new(chip, num))
    }

    pub fn generate_analog_output(&self, chip: u8, num: u8) -> Box<AnalogOutput> {
        Box::new(LibBeagleBonePwm::new(chip, num))
    }

    pub fn generate_digital_input(&self, num: u8) -> Box<DigitalInput> {
        let mut pin = GpioPinout::new(num);
        pin.set_input();
        Box::new(pin)
    }

    pub fn generate_digital_output(&self, num: u8) -> Box<DigitalOutput> {
        let mut pin = GpioPinout::new(num);
        pin.set_output();
        Box::new(pin)
    }
}