use libbeaglebone::pwm::PWM;

use crate::pinouts::analog::output::AnalogOutput;
use crate::pinouts::analog::output::PwmOutput;
use libbeaglebone::enums::DeviceState;

pub struct LibBeagleBonePwm {
    pwm: PWM,
    period: u32,
}

impl AnalogOutput for LibBeagleBonePwm {
    fn set_value(&mut self, val: f32) {
        if self.pwm.set_duty_cycle((val * self.period as f32) as u32).is_err() {
            error!("Failed to set value, re-exporting PWM!");
            if self.pwm.set_export(DeviceState::Exported).is_err() {
                error!("Failed to export PWM!");
            } else {
                info!("Successfully re-exported PWM")
            }
        }
    }
}

impl PwmOutput for LibBeagleBonePwm {
    fn set_pulse_duty_cycle(&mut self, val: u32) {
        if self.pwm.set_duty_cycle(val).is_err() {
            error!("Failed to set duty cycle, re-exporting PWM!");
            if self.pwm.set_export(DeviceState::Exported).is_err() {
                error!("Failed to export PWM!");
            } else {
                info!("Successfully re-exported PWM")
            }
        }
    }

    fn set_period(&mut self, val: u32) {
        if self.pwm.set_period(val).is_err() {
            error!("Failed to set period, re-exporting PWM!");
            if self.pwm.set_export(DeviceState::Exported).is_err() {
                error!("Failed to export PWM!");
            } else {
                info!("Successfully re-exported PWM")
            }
        } else {
            self.period = val;
        }
    }
}

impl LibBeagleBonePwm {
    pub fn new(chip: u8, num: u8) -> Self {
        let pwm = PWM::new(chip, num);
        if pwm.set_export(DeviceState::Exported).is_err() {
            error!("Failed to export PWM {}, {}!", chip, num);
        } else {
            info!("Successfully exported PWM")
        }
        Self {
            pwm,
            period: 20_000
        }
    }
}