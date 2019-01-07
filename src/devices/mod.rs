use std::process::Command;

pub mod motor_controllers;
pub mod sensors;

pub fn enable_pwm() {
    // Enable PWM Drivers
    Command::new("echo ").
        arg("am33xx_pwm")
        .arg(">")
        .arg("/sys/devices/platform/bone_capemgr/slots")
        .output()
        .expect("Failed to enable am33xx_pwm driver!");

    Command::new("echo ").
        arg("cape-universal")
        .arg(">")
        .arg("/sys/devices/platform/bone_capemgr/slots")
        .output()
        .expect("Failed to enable universal cape driver!");

    // Export chip
    Command::new("echo ").
        arg("1")
        .arg(">")
        .arg("/sys/class/pwm/pwmchip6/export")
        .output()
        .expect("Failed to export chip!");
}