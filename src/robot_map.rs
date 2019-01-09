///////////////////////////////////////////////////////////////////////////////////////////////////
// Drive Train Motor Controller Pins
///////////////////////////////////////////////////////////////////////////////////////////////////
pub const FRONT_LEFT_PWM_CHIP: u32 = 3; // P9.14
pub const FRONT_RIGHT_PWM_CHIP: u32 = 4; // P9.16
pub const REAR_LEFT_PWM_CHIP: u32 = 4; // P8.19
pub const REAR_RIGHT_PWM_CHIP: u32 = 6; // P8.13

pub const FRONT_LEFT_PWM_NUMBER: u32 = 1; //?
pub const FRONT_RIGHT_PWM_NUMBER: u32 = 1; //?
pub const REAR_LEFT_PWM_NUMBER: u32 = 1; //?
pub const REAR_RIGHT_PWM_NUMBER: u32 = 1; // GOOD

pub const FRONT_LEFT_DIRECTION: u64 = 60; // P9.12
pub const FRONT_RIGHT_DIRECTION: u64 = 66; // P8.7
pub const REAR_LEFT_DIRECTION: u64 = 69; // P9.9
pub const REAR_RIGHT_DIRECTION: u64 = 45; // P9.11

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

pub const ADDRESS: &str = "0.0.0.0";
pub const PORT: u16 = 2401;