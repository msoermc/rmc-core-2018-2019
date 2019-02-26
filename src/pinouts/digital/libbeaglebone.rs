use libbeaglebone::enums::DeviceState;
use libbeaglebone::gpio::GPIO;
use libbeaglebone::gpio::PinState;

use crate::pinouts::digital::input::DigitalInput;
use crate::pinouts::digital::output::DigitalOutput;
use libbeaglebone::gpio::PinDirection;
use std::path::Prefix::DeviceNS;
use libbeaglebone::pins::Pin;

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
    pub fn new(pin_number: Pin) -> Self {
        let pin = GPIO::new(pin_number);
        pin.set_export(DeviceState::Exported).unwrap();
        Self {
            pin,
        }
    }

    pub fn set_output(&mut self) {
        if let Err(error) = self.pin.set_direction(PinDirection::Out) {
            error!("{}", error);
        }
    }

    pub fn set_input(&mut self) {
        if let Err(error) = self.pin.set_direction(PinDirection::In) {
            error!("{}", error);
        }
    }
}

impl Drop for GpioPinout {
    fn drop(&mut self) {
        if let Err(e) = self.pin.set_export(DeviceState::Unexported) {
            error!("{}", e);
        }
    }
}