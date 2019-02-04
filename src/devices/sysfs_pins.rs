use sysfs_gpio;

use super::DigitalOutput;

pub struct SysfsPin {
    pin: sysfs_gpio::Pin
}

impl DigitalOutput for SysfsPin {
    fn set_value(&mut self, val: bool) {
        loop {
            if self.pin.set_value(val as u8).is_ok() {
                break;
            } else {
                warn!("Failed to set sysfs pin {}!", self.pin.get_pin_num());
                if self.pin.export().is_err() {
                    error!("Failed to reexport sysfs pin {}!", self.pin.get_pin_num());
                }
            }
        }
    }
}

impl SysfsPin {
    pub fn new(pin_num: u64) -> Self {
        Self {
            pin: sysfs_gpio::Pin::new(pin_num)
        }
    }
}

impl Drop for SysfsPin {
    fn drop(&mut self) {
        if self.pin.unexport().is_err() {
            error!("Failed to unexport sysfs pin {}!", self.pin.get_pin_num());
        }
    }
}