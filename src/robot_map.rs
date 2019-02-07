///////////////////////////////////////////////////////////////////////////////////////////////////
// Drive Train Motor Controller Pins
///////////////////////////////////////////////////////////////////////////////////////////////////

use slog::Level;

// P9.14
pub const FRONT_LEFT_DRIVE_STRING: &str = "P9_14";
pub const FRONT_LEFT_PWM_CHIP: u32 = 3;
pub const FRONT_LEFT_PWM_NUMBER: u32 = 0;

// P9.16
pub const FRONT_RIGHT_DRIVE_STRING: &str = "P9_16";
pub const FRONT_RIGHT_PWM_CHIP: u32 = 3;
pub const FRONT_RIGHT_PWM_NUMBER: u32 = 1;

// P8.19
pub const REAR_LEFT_DRIVE_STRING: &str = "P8_19";
pub const REAR_LEFT_PWM_CHIP: u32 = 6;
pub const REAR_LEFT_PWM_NUMBER: u32 = 1;

// P8.13
pub const REAR_RIGHT_DRIVE_STRING: &str = "P8_13";
pub const REAR_RIGHT_PWM_CHIP: u32 = 6;
pub const REAR_RIGHT_PWM_NUMBER: u32 = 0;

// P9.12
pub const FRONT_LEFT_DIRECTION_STRING: &str = "P9_12";
pub const FRONT_LEFT_DIRECTION: u64 = 60;

// P8.7
pub const FRONT_RIGHT_DIRECTION_STRING: &str = "P8_7";
pub const FRONT_RIGHT_DIRECTION: u64 = 66;

// P8.17
pub const REAR_LEFT_DIRECTION_STRING: &str = "P8_17";
pub const REAR_LEFT_DIRECTION: u64 = 27;

// P8.11
pub const REAR_RIGHT_DIRECTION_STRING: &str = "P8_11";
pub const REAR_RIGHT_DIRECTION: u64 = 45;

/// The address for the tcp server used to communicate with the driver station.
/// Zero indicates that the server will accept connections from any IP.
pub const ADDRESS: &str = "0.0.0.0";

/// The port used for communicating with the driver station.
pub const PORT: u16 = 2401;

/// The path of the folder which logs will be kept in.
/// If the folder does not exist, the program will create it.
pub const LOG_PATH: &str = "./rmc.log";

pub const LOG_FILTER_LEVEL: Level = Level::Info;

pub const DIGGING_RATE: f32 = 1.0;

pub const MH_ACTUATOR_RATE: f32 = 1.0;

pub const DUMPING_RATE: f32 = 1.0;

pub const DUMPER_RESET_RATE: f32 = -1.0;