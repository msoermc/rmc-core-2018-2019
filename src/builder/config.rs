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
use crate::pinouts::enable_pins;

pub struct RobotAssemblyBuilder {
    dumper: Box<SubsystemFactory<Dumper>>,
    intake: Box<SubsystemFactory<Intake>>,
    drive: Box<SubsystemFactory<DriveTrain>>,
    state: Arc<GlobalRobotState>,
    bench: Option<ControllerBench>,
    io: Rc<IoFactory>,
    pin_enabled_status: bool,
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
            pin_enabled_status: false,
        }
    }

    pub fn get_state(&self) -> Arc<GlobalRobotState> {
        self.state.clone()
    }

    pub fn with_test(&mut self) -> &mut Self {
        self.with_test_drive().with_test_dumper().with_test_ladder()
    }

    pub fn with_production(&mut self) -> &mut Self {
        self.with_production_drive().with_production_dumper().with_production_ladder()
    }

    pub fn with_bench(&mut self) {
        let bench = ControllerBench::new(self.state.get_cycle_counter(), self.state.get_cycles_per_second());
        self.bench = Some(bench);
    }

    pub fn with_production_drive(&mut self) -> &mut Self {
        self.drive = Box::new(ProductionDriveFactory::new(self.state.clone(), self.io.clone()));
        self.with_pinouts()
    }

    pub fn with_test_drive(&mut self) -> &mut Self {
        self.drive = Box::new(TestDriveFactory::new(self.state.clone()));
        self
    }

    pub fn with_production_dumper(&mut self) -> &mut Self {
        self.dumper = Box::new(ProductionDumperFactory::new(self.state.clone(), self.io.clone()));
        self.with_pinouts()
    }

    pub fn with_test_dumper(&mut self) -> &mut Self {
        self.dumper = Box::new(TestDumperFactory::new(self.state.clone()));
        self
    }

    pub fn with_production_ladder(&mut self) -> &mut Self {
        self.intake = Box::new(ProductionIntakeFactory::new(self.state.clone(), self.io.clone()));
        self.with_pinouts()
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

    pub fn get_drive_factory(&self) -> String {
        self.drive.to_string()
    }

    pub fn get_dumper_factory(&self) -> String {
        self.dumper.to_string()
    }

    pub fn get_intake_factory(&self) -> String {
        self.intake.to_string()
    }

    fn with_pinouts(&mut self) -> &mut Self {
        if !self.get_pin_status() {
            if enable_pins().is_err() {
                error!("Failed to enable pins!");
            } else {
                info!("Enabled pins!");
                self.pin_enabled_status = true;
            }
        } else {
            info!("Pins already enabled, skipping enable");
        }
        self
    }

    fn get_pin_status(&self) -> bool {
        self.pin_enabled_status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_production() {
        let mut builder = RobotAssemblyBuilder::new();
        builder.with_production();
        assert_eq!("production drive", builder.get_drive_factory());
        assert_eq!("production dumper", builder.get_dumper_factory());
        assert_eq!("production intake", builder.get_intake_factory());
        assert_eq!(true, builder.get_pin_status());
    }

    #[test]
    fn global_test() {
        let mut builder = RobotAssemblyBuilder::new();
        builder.with_test();
        assert_eq!("test drive", builder.get_drive_factory());
        assert_eq!("test dumper", builder.get_dumper_factory());
        assert_eq!("test intake", builder.get_intake_factory());
        assert_eq!(false, builder.get_pin_status());
    }

    #[test]
    fn global_print() {
        let builder = RobotAssemblyBuilder::new();
        assert_eq!("print drive", builder.get_drive_factory());
        assert_eq!("print dumper", builder.get_dumper_factory());
        assert_eq!("print intake", builder.get_intake_factory());
        assert_eq!(false, builder.get_pin_status());
    }
}