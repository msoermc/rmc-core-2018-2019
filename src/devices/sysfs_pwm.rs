use sysfs_pwm;

use super::AnalogOutput;

const PERIOD_NS: u32 = 20_000;

pub struct SysfsPwm {
    pwm: sysfs_pwm::Pwm,
    chip: u32,
    number: u32,
}

impl AnalogOutput for SysfsPwm {
    fn set_value(&mut self, val: f32) {
        let pwm_out = f32::abs(val * PERIOD_NS as f32) as u32;
        let mut fail_count = 0;
        while self.pwm.set_duty_cycle_ns(pwm_out).is_err() {
            fail_count += 1;
            warn!("Failed to set duty cycle for sysfs pwm <{},{}>!", self.chip, self.number);
            if self.pwm.export().is_err() {
                warn!("Failed to reexport sysfs pwm <{},{}>!", self.chip, self.number);
            } else {
                if self.pwm.set_period_ns(PERIOD_NS).is_err() {
                    warn!("Failed to export sysfs pwm <{},{}>!", self.chip, self.number);
                }
            }

            if fail_count >= 7 {
                error!("Failed to set value for sysfs pwm <{},{}>!", self.chip, self.number);
            }
        }
    }
}

impl SysfsPwm {
    pub fn create(chip: u32, number: u32) -> sysfs_pwm::Result<Self> {
        let pwm = sysfs_pwm::Pwm::new(chip, number)?;

        if pwm.export().is_err() {
            error!("Failed to export sysfs pwm <{},{}>!", chip, number);
        }

        if pwm.set_period_ns(PERIOD_NS).is_err() {
            error!("Failed to export sysfs pwm <{},{}>!", chip, number);
        }

        Ok(Self {
            pwm,
            chip,
            number,
        })
    }
}