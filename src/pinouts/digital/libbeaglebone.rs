use std::path::Prefix::DeviceNS;

use libbeaglebone::enums::DeviceState;
use libbeaglebone::gpio::GPIO;
use libbeaglebone::gpio::PinDirection;
use libbeaglebone::gpio::PinState;
use libbeaglebone::pins::Pin;

use crate::pinouts::digital::input::DigitalInput;
use crate::pinouts::digital::output::DigitalOutput;

pub struct GpioPinout {
    pin: GPIO,
}

impl DigitalOutput for GpioPinout {
    fn set_value(&mut self, val: bool) {
        let mut set_pin = |state| {
            if let Err(e) = self.pin.write(state) {
                error!("{}", e);
                self.set_output();
            }
        };
        if val {
            set_pin(PinState::High);
        } else {
            set_pin(PinState::Low);
        }
    }
}

impl DigitalInput for GpioPinout {
    fn get_value(&self) -> Option<bool> {
        match self.pin.read() {
            Ok(val) => {
                Some(PinState::High == val)
            },
            Err(e) => {
                error!("{}", e);
                None
            },
        }
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

    pub fn set_output_twice(&mut self) {
        if let Err(error) = self.pin.set_direction(PinDirection::Out) {
            self.set_output();
        }
    }

    pub fn set_input(&mut self) {
        if let Err(error) = self.pin.set_direction(PinDirection::In) {
            error!("{}", error);
        }
    }

    pub fn set_input_twice(&mut self) {
        if let Err(error) = self.pin.set_direction(PinDirection::In) {
            self.set_input();
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