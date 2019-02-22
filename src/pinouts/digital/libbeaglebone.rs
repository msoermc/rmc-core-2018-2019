use libbeaglebone::enums::DeviceState;
use libbeaglebone::gpio::GPIO;
use libbeaglebone::gpio::PinState;

use crate::pinouts::digital::input::DigitalInput;
use crate::pinouts::digital::output::DigitalOutput;

pub struct GpioPinout {
    pin: GPIO,
}

impl DigitalOutput for GpioPinout {
    fn set_value(&mut self, val: bool) -> Result<(), String> {
        if val {
            if let Err(e) = self.pin.write(PinState::High) {
                self.pin.set_export(DeviceState::Exported).unwrap();
                error!("{:?}", e);
                Err(format!("{:?}", e))
            } else {
                Ok(())
            }
        } else {
            if let Err(e) = self.pin.write(PinState::Low) {
                self.pin.set_export(DeviceState::Exported).unwrap();
                error!("{:?}", e);
                Err(format!("{:?}", e))
            } else {
                Ok(())
            }
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
        Self {
            pin: GPIO::new(pin_number),
        }
    }
}