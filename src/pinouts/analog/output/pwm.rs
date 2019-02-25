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
        self.pwm.set_duty_cycle((val * self.period as f32) as u32).unwrap();
    }
}

impl PwmOutput for LibBeagleBonePwm {
    fn set_pulse_duty_cycle(&mut self, val: u32) {
        self.pwm.set_duty_cycle(val).unwrap();
    }

    fn set_period(&mut self, val: u32) {
        self.pwm.set_period(val).unwrap();
        self.period = val;
    }
}

impl LibBeagleBonePwm {
    pub fn new(chip: u8, num: u8) -> Self {
        let pwm = PWM::new(chip, num);
        pwm.set_export(DeviceState::Exported).expect("Failed to export PWM!");
        Self {
            pwm,
            period: 20_000
        }
    }
}