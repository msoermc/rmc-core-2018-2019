use crate::mechatronics::controller::RobotController;

pub trait RobotCommand: Send {
    fn execute(&self, controller: &mut RobotController);
}

pub struct KillCommand {}

impl RobotCommand for KillCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_life().kill();
        controller.get_dumper().stop();
        controller.get_drive_train().brake();
        controller.get_intake().stop_actuators();
        controller.get_intake().stop_digging();
    }
}

pub struct DriveCommand {
    left: f32,
    right: f32,
}

impl RobotCommand for DriveCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_drive_train().drive(self.left, self.right)
    }
}

pub struct ReviveCommand {}

impl RobotCommand for ReviveCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_life().revive();
    }
}

pub struct DriveSwitchCommand {}

impl RobotCommand for DriveSwitchCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_intake().disable();
        controller.get_dumper().disable();
        controller.get_drive_train().enable();
    }
}

pub struct IntakeSwitchCommand {}

impl RobotCommand for IntakeSwitchCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_dumper().disable();
        controller.get_drive_train().disable();
        controller.get_intake().enable();
    }
}

pub struct DumperSwitchCommand {}

impl RobotCommand for DumperSwitchCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_drive_train().disable();
        controller.get_intake().disable();
        controller.get_dumper().enable();
    }
}

pub struct BrakeCommand {}

impl RobotCommand for BrakeCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_drive_train().brake();
    }
}

pub struct DigCommand {}

impl RobotCommand for DigCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_intake().dig();
    }
}

pub struct StopDiggerCommand {}

impl RobotCommand for StopDiggerCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_intake().stop_digging();
    }
}

pub struct DumpCommand {}

impl RobotCommand for DumpCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_dumper().dump();
    }
}

pub struct StopDumperCommand {}

impl RobotCommand for StopDumperCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_dumper().stop();
    }
}

pub struct ResetDumperCommand {}

impl RobotCommand for ResetDumperCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_dumper().reset();
    }
}

pub struct StopActuatorsCommand {}

impl RobotCommand for StopActuatorsCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_intake().stop_actuators();
    }
}

pub struct RaiseActuatorsCommand {}

impl RobotCommand for RaiseActuatorsCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_intake().raise();
    }
}

pub struct LowerActuatorsCommand {}

impl RobotCommand for LowerActuatorsCommand {
    fn execute(&self, controller: &mut RobotController) {
        controller.get_intake().lower();
    }
}

pub struct RobotCommandFactory {}

impl RobotCommandFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_drive_command(&self, left: f32, right: f32) -> Option<DriveCommand> {
        let check = |x| x <= 1.0 && x >= -1.0;
        if check(left) && check(right) {
            Some(DriveCommand { left, right })
        } else {
            None
        }
    }

    pub fn generate_kill_command(&self) -> KillCommand {
        KillCommand {}
    }

    pub fn generate_revive_command(&self) -> ReviveCommand {
        ReviveCommand {}
    }

    pub fn generate_drive_switch_command(&self) -> DriveSwitchCommand {
        DriveSwitchCommand {}
    }

    pub fn generate_intake_switch_command(&self) -> IntakeSwitchCommand {
        IntakeSwitchCommand {}
    }

    pub fn generate_dumper_switch_command(&self) -> DumperSwitchCommand {
        DumperSwitchCommand {}
    }

    pub fn generate_brake_command(&self) -> BrakeCommand {
        BrakeCommand {}
    }

    pub fn generate_dig_command(&self) -> DigCommand {
        DigCommand {}
    }

    pub fn generate_stop_digger_command(&self) -> StopDiggerCommand {
        StopDiggerCommand {}
    }

    pub fn generate_dump_command(&self) -> DumpCommand {
        DumpCommand {}
    }

    pub fn generate_stop_dumper_command(&self) -> StopDumperCommand {
        StopDumperCommand {}
    }

    pub fn generate_reset_dumper_command(&self) -> ResetDumperCommand {
        ResetDumperCommand {}
    }

    pub fn generate_raise_actuators_command(&self) -> RaiseActuatorsCommand {
        RaiseActuatorsCommand {}
    }

    pub fn generate_lower_actuators_command(&self) -> LowerActuatorsCommand {
        LowerActuatorsCommand {}
    }

    pub fn generate_stop_actuators_command(&self) -> StopActuatorsCommand {
        StopActuatorsCommand {}
    }
}