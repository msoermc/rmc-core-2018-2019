pub mod interface;
mod drive_train;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RobotControllerCommand {
    Drive(f32, f32),
    Stop,
    Enable,
    Disable,
}

