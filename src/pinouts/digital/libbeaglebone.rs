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
    fn get_value(&self) -> bool {
        match self.pin.read() {
            Ok(val) => {
                PinState::High == val
            }
            Err(e) => {
                error!("{}", e);
                false
            }
        }
    }
}

impl GpioPinout {
    pub fn new(pin_number: Pin) -> Self {
        let pin = GPIO::new(pin_number);
        if let Err(e) = pin.set_export(DeviceState::Exported) {
            error!("{}", e);
        }
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
        if self.pin.set_direction(PinDirection::Out).is_err() {
            self.set_output();
        }
    }

    pub fn set_input(&mut self) {
        if let Err(error) = self.pin.set_direction(PinDirection::In) {
            error!("{}", error);
        }
    }

    pub fn set_input_twice(&mut self) {
        if self.pin.set_direction(PinDirection::In).is_err() {
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