use std::sync::Arc;

use crate::mechatronics::drive_train::state::DriveTrainStateInstance;
use crate::mechatronics::drive_train::state::GlobalDriveTrainState;
use crate::mechatronics::material_handling::bucket_ladder::state::GlobalIntakeState;
use crate::mechatronics::material_handling::bucket_ladder::state::IntakeStateInstance;
use crate::mechatronics::material_handling::dumper::state::DumperStateInstance;
use crate::mechatronics::material_handling::dumper::state::GlobalDumperState;
use crate::status::life::GlobalLifeState;
use crate::status::life::LifeStateInstance;

pub struct GlobalRobotState {
    life: Arc<GlobalLifeState>,
    drive: Arc<GlobalDriveTrainState>,
    dumper: Arc<GlobalDumperState>,
    intake: Arc<GlobalIntakeState>,
}

impl GlobalRobotState {
    pub fn new() -> Self {
        Self {
            life: Arc::new(GlobalLifeState::new()),
            drive: Arc::new(GlobalDriveTrainState::new()),
            dumper: Arc::new(GlobalDumperState::new()),
            intake: Arc::new(GlobalIntakeState::new()),
        }
    }

    pub fn get_life(&self) -> Arc<GlobalLifeState> {
        self.life.clone()
    }

    pub fn get_drive(&self) -> Arc<GlobalDriveTrainState> {
        self.drive.clone()
    }

    pub fn get_dumper(&self) -> Arc<GlobalDumperState> {
        self.dumper.clone()
    }

    pub fn get_intake(&self) -> Arc<GlobalIntakeState> {
        self.intake.clone()
    }

    pub fn get_current_state(&self) -> RobotStateInstance {
        RobotStateInstance::new(
            self.life.get_current_state(),
            self.drive.get_current_state(),
            self.dumper.get_current_state(),
            self.intake.get_current_state()
        )
    }
}

#[derive(Serialize)]
pub struct RobotStateInstance {
    life: LifeStateInstance,
    drive: DriveTrainStateInstance,
    dumper: DumperStateInstance,
    intake: IntakeStateInstance,
}

impl RobotStateInstance {
    pub fn new(life: LifeStateInstance, drive: DriveTrainStateInstance, dumper: DumperStateInstance,
               intake: IntakeStateInstance) -> Self {
        Self {
            life,
            drive,
            dumper,
            intake,
        }
    }
}