use libbeaglebone::enums::DeviceState;
use libbeaglebone::pwm::PWM;
use libbeaglebone::pwm::PWMState;

use crate::pinouts::analog::output::AnalogOutput;
use crate::pinouts::analog::output::PwmOutput;

pub struct LibBeagleBonePwm {
    pwm: PWM,
    period: u32,
}

impl AnalogOutput for LibBeagleBonePwm {
    fn set_value(&mut self, val: f32) {
        if let Err(e) = self.pwm.set_duty_cycle((val * self.period as f32) as u32) {
            error!("{}", e);
        }
    }
}

impl PwmOutput for LibBeagleBonePwm {
    fn set_pulse_duty_cycle(&mut self, val: u32) {
        if let Err(e) = self.pwm.set_duty_cycle(val) {
            error!("{}", e);
        }

        if let Err(e) = self.pwm.set_period(val) {
            error!("{}", e);
        }
    }

    fn set_period(&mut self, val: u32) {
        if let Err(e) = self.pwm.set_period(val) {
            error!("{}", e);
        }

        self.period = val;
    }
}

impl LibBeagleBonePwm {
    pub fn new(chip: u8, num: u8) -> Self {
        let mut pwm = PWM::new(chip, num);

        if let Err(e) = pwm.set_period(20_000_000) {
            error!("{}", e);
        }
        if let Err(e) = pwm.set_export(DeviceState::Exported) {
            error!("{}", e);
        }
        if let Err(e) = pwm.set_state(PWMState::Enabled) {
            error!("{}", e);
        }

        Self {
            pwm,
            period: 20_000,
        }
    }
}