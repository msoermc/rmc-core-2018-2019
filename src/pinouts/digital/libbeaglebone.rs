use libbeaglebone::enums::DeviceState;
use libbeaglebone::gpio::GPIO;
use libbeaglebone::gpio::PinState;

use crate::pinouts::digital::input::DigitalInput;
use crate::pinouts::digital::output::DigitalOutput;
use libbeaglebone::gpio::PinDirection;

pub struct GpioPinout {
    pin: GPIO,
}

impl DigitalOutput for GpioPinout {
    fn set_value(&mut self, val: bool) {
        if val {
            self.pin.write(PinState::High).unwrap()
        } else {
            self.pin.write(PinState::Low).unwrap()
        }
    }
}

impl DigitalInput for GpioPinout {
    fn get_value(&self) -> Option<bool> {
        self.pin.read().ok().map(|val| val == PinState::High)
    }
}

impl GpioPinout {
    pub fn new(pin_number: u8) -> Self {
        let pin = GPIO::new(pin_number);
        pin.set_export(DeviceState::Exported).unwrap();
        if let Err(error) = pin.set_direction(PinDirection::Out) {
            error!("{}", error);
        }
        Self {
            pin,
        }
    }
}