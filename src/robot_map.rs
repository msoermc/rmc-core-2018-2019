use libbeaglebone::pins::Pin;
use slog::Level;

// P9.14
pub const FRONT_LEFT_PWM_CHIP: u8 = 3;
pub const FRONT_LEFT_PWM_NUMBER: u8 = 0;

// P9.16
pub const FRONT_RIGHT_PWM_CHIP: u8 = 3;
pub const FRONT_RIGHT_PWM_NUMBER: u8 = 1;

// P8.19
pub const REAR_LEFT_PWM_CHIP: u8 = 6;
pub const REAR_LEFT_PWM_NUMBER: u8 = 1;

// P8.13
pub const REAR_RIGHT_PWM_CHIP: u8 = 6;
pub const REAR_RIGHT_PWM_NUMBER: u8 = 0;

// P9.22
pub const DUMPER_PWM_CHIP: u8 = 1;
pub const DUMPER_PWM_NUM: u8 = 0;

// P9.21
pub const DIGGER_PWM_CHIP: u8 = 1;
pub const DIGGER_PWM_NUM: u8 = 1;

// P9.42
pub const ACTUATOR_PWM_CHIP: u8 = 0;
pub const ACTUATOR_PWM_NUM: u8 = 0;

pub const FRONT_LEFT_DIRECTION: Pin = Pin::GPIO_P9_12;
pub const FRONT_RIGHT_DIRECTION: Pin = Pin::GPIO_P8_9;
pub const REAR_LEFT_DIRECTION: Pin = Pin::GPIO_P8_17;
pub const REAR_RIGHT_DIRECTION: Pin = Pin::GPIO_P8_11;

pub const LEFT_UPPER_ACTUATOR_LIMIT: Pin = Pin::GPIO_P9_15;
pub const LEFT_LOWER_ACTUATOR_LIMIT: Pin = Pin::GPIO_P9_30;
pub const RIGHT_UPPER_ACTUATOR_LIMIT: Pin = Pin::GPIO_P8_7;
pub const RIGHT_LOWER_ACTUATOR_LIMIT: Pin = Pin::GPIO_P8_8;

pub const DUMPER_UPPER_ACTUATOR_LIMIT: Pin = Pin::GPIO_P8_15;
pub const DUMPER_LOWER_ACTUATOR_LIMIT: Pin = Pin::GPIO_P8_16;

/// The path of the folder which logs will be kept in.
/// If the folder does not exist, the program will create it.
pub const LOG_PATH: &str = "./rmc.log";

/// The lowest level of logs which will be displayed to the user.
pub const LOG_FILTER_LEVEL: Level = Level::Info;

/// The speed given to the motors as they dig.
pub const DIGGING_RATE: f32 = 0.25;

/// The speed at which the actuators move.
pub const MH_ACTUATOR_RATE: f32 = 0.25;

/// The speed used by the motors to dump material.
pub const DUMPING_RATE: f32 = 0.25;

/// The speed used by the dumper to reset it's position.
pub const DUMPER_RESET_RATE: f32 = -DUMPING_RATE;

// Thresholds for current levels (Currently placeholder)
pub const NORMAL_CURRENT: f32 = 1.8;
pub const BROWN_CURRENT: f32 = 2.0;
pub const CRITICAL_CURRENT: f32 = 3.3;