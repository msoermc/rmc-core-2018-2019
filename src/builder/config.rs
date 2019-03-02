use std::rc::Rc;
use std::sync::Arc;

use crate::benchmarking::ControllerBench;
use crate::builder::assembly::RobotAssembler;
use crate::builder::factories::drive::PrintDriveFactory;
use crate::builder::factories::drive::ProductionDriveFactory;
use crate::builder::factories::drive::TestDriveFactory;
use crate::builder::factories::dumper::PrintDumperFactory;
use crate::builder::factories::dumper::ProductionDumperFactory;
use crate::builder::factories::dumper::TestDumperFactory;
use crate::builder::factories::intake::PrintIntakeFactory;
use crate::builder::factories::intake::ProductionIntakeFactory;
use crate::builder::factories::intake::TestIntakeFactory;
use crate::builder::factories::SubsystemFactory;
use crate::mechatronics::bucket_ladder::Intake;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::dumper::Dumper;
use crate::pinouts::factories::IoFactory;
use crate::status::robot_state::GlobalRobotState;

pub struct RobotAssemblyBuilder {
    dumper: Box<SubsystemFactory<Dumper>>,
    intake: Box<SubsystemFactory<Intake>>,
    drive: Box<SubsystemFactory<DriveTrain>>,
    state: Arc<GlobalRobotState>,
    bench: Option<ControllerBench>,
    io: Rc<IoFactory>,
}

impl RobotAssemblyBuilder {
    pub fn new() -> Self {
        let state = Arc::new(GlobalRobotState::new());

        Self {
            dumper: Box::new(PrintDumperFactory::new(state.clone())),
            intake: Box::new(PrintIntakeFactory::new(state.clone())),
            drive: Box::new(PrintDriveFactory::new(state.clone())),
            state,
            bench: None,
            io: Rc::new(IoFactory::new()),
        }
    }

    pub fn get_state(&self) -> Arc<GlobalRobotState> {
        self.state.clone()
    }

    pub fn with_test(&mut self) -> &mut Self {
        self.with_test_drive().with_test_dumper().with_test_ladder()
    }

    pub fn with_bench(&mut self) {
        let bench = ControllerBench::new(self.state.get_cycle_counter(), self.state.get_cycles_per_second());
        self.bench = Some(bench);
    }

    pub fn with_production_drive(&mut self) -> &mut Self {
        self.drive = Box::new(ProductionDriveFactory::new(self.state.clone(), self.io.clone()));
        self
    }

    pub fn with_test_drive(&mut self) -> &mut Self {
        self.drive = Box::new(TestDriveFactory::new(self.state.clone()));
        self
    }

    pub fn with_production_dumper(&mut self) -> &mut Self {
        self.dumper = Box::new(ProductionDumperFactory::new(self.state.clone(), self.io.clone()));
        self
    }

    pub fn with_test_dumper(&mut self) -> &mut Self {
        self.dumper = Box::new(TestDumperFactory::new(self.state.clone()));
        self
    }

    pub fn with_production_ladder(&mut self) -> &mut Self {
        self.intake = Box::new(ProductionIntakeFactory::new(self.state.clone(), self.io.clone()));
        self
    }

    pub fn with_test_ladder(&mut self) -> &mut Self {
        self.intake = Box::new(TestIntakeFactory::new(self.state.clone()));
        self
    }

    pub fn generate(self) -> RobotAssembler {
        let dumper = self.dumper.produce();
        let drive = self.drive.produce();
        let intake = self.intake.produce();

        RobotAssembler::new(dumper, drive, intake, self.state, self.bench)
    }
}