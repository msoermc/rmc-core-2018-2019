use std::process;

use sysfs_gpio;

use super::DigitalOutput;
use std::io;

pub struct SysfsPin {
    pin: sysfs_gpio::Pin
}

impl DigitalOutput for SysfsPin {
    fn set_value(&mut self, val: bool) -> Result<(), String> {
        let mut counter: u8 = 0;
        loop {
            if self.pin.set_value(val as u8).is_ok() {
                return Ok(());
            } else {
                if counter >= 7 {
                    return Err("Failed to set sysfs pin and reexport!".to_owned());
                }
                counter += 1;
                warn!("Failed to set sysfs pin {}!", self.pin.get_pin_num());
                if self.pin.export().is_err() {
                    error!("Failed to reexport sysfs pin {}!", self.pin.get_pin_num());
                }
            }
        }
    }
}

impl SysfsPin {
    pub fn new(pin_num: u64, board_location: &str) -> io::Result<Self> {
        let config_command = process::Command::new("config-pin")
            .arg(board_location)
            .arg("gpio")
            .output();

        if let Err(e) = config_command {
            error!("Failed to configure pin {}! Error:\n{}", board_location, e);
            return Err(e);
        }

        let pin = sysfs_gpio::Pin::new(pin_num);

        Ok(Self {
            pin
        })
    }
}

impl Drop for SysfsPin {
    fn drop(&mut self) {
        if self.pin.unexport().is_err() {
            error!("Failed to unexport sysfs pin {}!", self.pin.get_pin_num());
        }
    }
}