use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::mechatronics::bucket_ladder::state::GlobalIntakeState;
use crate::mechatronics::bucket_ladder::state::IntakeStateInstance;
use crate::mechatronics::drive_train::state::DriveTrainStateInstance;
use crate::mechatronics::drive_train::state::GlobalDriveTrainState;
use crate::mechatronics::dumper::state::DumperStateInstance;
use crate::mechatronics::dumper::state::GlobalDumperState;
use crate::status::life::GlobalLifeState;
use crate::status::life::LifeStateInstance;

pub struct GlobalRobotState {
    life: Arc<GlobalLifeState>,
    drive: Arc<GlobalDriveTrainState>,
    dumper: Arc<GlobalDumperState>,
    intake: Arc<GlobalIntakeState>,
    cycles_per_sec: Arc<AtomicUsize>,
    cycle_counter: Arc<AtomicUsize>,
}

impl GlobalRobotState {
    pub fn new() -> Self {
        Self {
            life: Arc::new(GlobalLifeState::new()),
            drive: Arc::new(GlobalDriveTrainState::new()),
            dumper: Arc::new(GlobalDumperState::new()),
            intake: Arc::new(GlobalIntakeState::new()),
            cycles_per_sec: Arc::new(AtomicUsize::new(0)),
            cycle_counter: Arc::new(AtomicUsize::new(0)),
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
            self.intake.get_current_state(),
            self.cycles_per_sec.load(Ordering::SeqCst),
            self.cycle_counter.load(Ordering::SeqCst),
        )
    }

    pub fn get_cycles_per_second(&self) -> Arc<AtomicUsize> {
        self.cycles_per_sec.clone()
    }

    pub fn get_cycle_counter(&self) -> Arc<AtomicUsize> {
        self.cycle_counter.clone()
    }
}

#[derive(Serialize)]
pub struct RobotStateInstance {
    life: LifeStateInstance,
    drive: DriveTrainStateInstance,
    dumper: DumperStateInstance,
    intake: IntakeStateInstance,
    cycles_per_sec: usize,
    cycle_counter: usize,
}

impl RobotStateInstance {
    pub fn new(life: LifeStateInstance, drive: DriveTrainStateInstance, dumper: DumperStateInstance,
               intake: IntakeStateInstance, cycles_per_sec: usize, cycle_counter: usize) -> Self {
        Self {
            life,
            drive,
            dumper,
            intake,
            cycles_per_sec,
            cycle_counter,
        }
    }

    pub fn get_life(&self) -> &LifeStateInstance {
        &self.life
    }

    pub fn get_drive(&self) -> &DriveTrainStateInstance {
        &self.drive
    }

    pub fn get_dumper(&self) -> &DumperStateInstance {
        &self.dumper
    }

    pub fn get_intake(&self) -> &IntakeStateInstance {
        &self.intake
    }
}