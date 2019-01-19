///////////////////////////////////////////////////////////////////////////////////////////////////
// Drive Train Motor Controller Pins
///////////////////////////////////////////////////////////////////////////////////////////////////

// P9.14
pub const FRONT_LEFT_PWM_CHIP: u32 = 3;
pub const FRONT_LEFT_PWM_NUMBER: u32 = 0;

// P9.16
pub const FRONT_RIGHT_PWM_CHIP: u32 = 3;
pub const FRONT_RIGHT_PWM_NUMBER: u32 = 1;

// P8.19
pub const REAR_LEFT_PWM_CHIP: u32 = 6;
pub const REAR_LEFT_PWM_NUMBER: u32 = 1;

// P8.13
pub const REAR_RIGHT_PWM_CHIP: u32 = 6;
pub const REAR_RIGHT_PWM_NUMBER: u32 = 0;

// P9.12
pub const FRONT_LEFT_DIRECTION: u64 = 60;

// P8.7
pub const FRONT_RIGHT_DIRECTION: u64 = 66;

// P8.17
pub const REAR_LEFT_DIRECTION: u64 = 27;

// P8.11
pub const REAR_RIGHT_DIRECTION: u64 = 45;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Motor ID
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MotorID {
    DriveTrainFrontLeft,
    DriveTrainFrontRight,
    DriveTrainRearLeft,
    DriveTrainRearRight,
}

impl ToString for MotorID {
    fn to_string(&self) -> String {
        // TODO Implement motor strings
        unimplemented!()
    }
}

/// The address for the tcp server used to communicate with the driver station.
/// Zero indicates that the server will accept connections from any IP.
pub const ADDRESS: &str = "0.0.0.0";

/// The port used for communicating with the driver station.
pub const PORT: u16 = 2401;

/// The path of the folder which logs will be kept in.
/// If the folder does not exist, the program will create it.
pub const LOG_PATH: &str = "./rmc.log";