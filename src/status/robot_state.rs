use std::sync::Arc;

use crate::mechatronics::drive_train::state::GlobalDriveTrainState;
use crate::mechatronics::material_handling::bucket_ladder::state::GlobalIntakeState;
use crate::mechatronics::material_handling::dumper::state::GlobalDumperState;
use crate::status::life::GlobalLifeStatus;

pub struct RobotState {
    life: GlobalLifeStatus,
    drive: Arc<GlobalDriveTrainState>,
    dumper: Arc<GlobalDumperState>,
    intake: Arc<GlobalIntakeState>,
}

impl RobotState {
    pub fn new() -> Self {
        Self {
            life: GlobalLifeStatus::new(),
            drive: Arc::new(GlobalDriveTrainState::new()),
            dumper: Arc::new(GlobalDumperState::new()),
            intake: Arc::new(GlobalIntakeState::new()),
        }
    }

    pub fn get_life(&self) -> GlobalLifeStatus {
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
}