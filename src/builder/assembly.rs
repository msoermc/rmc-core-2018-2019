use std::sync::Arc;
use std::sync::mpsc::sync_channel;

use crate::benchmarking::ControllerBench;
use crate::builder::launch::RobotLauncher;
use crate::comms;
use crate::mechatronics::bucket_ladder::Intake;
use crate::mechatronics::commands::RobotCommandFactory;
use crate::mechatronics::controller::RobotController;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::dumper::Dumper;
use crate::mechatronics::RobotMessenger;
use crate::status::robot_state::GlobalRobotState;

pub struct RobotAssembler {
    dumper: Dumper,
    drive: DriveTrain,
    intake: Intake,
    state: Arc<GlobalRobotState>,
    bench: Option<ControllerBench>,
}

impl RobotAssembler {
    pub fn new(dumper: Dumper, drive: DriveTrain, intake: Intake, state: Arc<GlobalRobotState>, bench: Option<ControllerBench>) -> Self {
        Self {
            dumper,
            drive,
            intake,
            state,
            bench,
        }
    }

    pub fn assemble(self) -> RobotLauncher {
        let (controller_sender, controller_receiver) = sync_channel(20);

        let command_factory = RobotCommandFactory::new();

        let robot_view = RobotMessenger::new(controller_sender);
        let bfr = comms::stage(robot_view, self.state.clone(), command_factory);

        let robot_controller = RobotController::new(controller_receiver, self.drive, self.dumper, self.intake, self.state.get_life(), self.state.get_cycle_counter());

        RobotLauncher::new(robot_controller, bfr, self.bench)
    }
}

