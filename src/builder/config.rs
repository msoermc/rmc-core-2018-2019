use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use libbeaglebone::pins::Pin;

use crate::benchmarking::ControllerBench;
use crate::builder::assembly::RobotAssembler;
use crate::builder::factories::digital_monitor::DigitalMonitorFactory;
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
use crate::framework::{CompositeRunnable, Runnable};
use crate::mechatronics::bucket_ladder::Intake;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::dumper::Dumper;
use crate::pinouts::digital::input::DigitalInput;
use crate::pinouts::enable_pins;
use crate::pinouts::factories::IoFactory;
use crate::robot_map::*;
use crate::status::robot_state::GlobalRobotState;

pub struct RobotAssemblyBuilder {
    dumper: Box<SubsystemFactory<Dumper>>,
    intake: Box<SubsystemFactory<Intake>>,
    drive: Box<SubsystemFactory<DriveTrain>>,
    right_upper_limit: Option<Box<SubsystemFactory<Box<Runnable>>>>,
    right_lower_limit: Option<Box<SubsystemFactory<Box<Runnable>>>>,
    left_upper_limit: Option<Box<SubsystemFactory<Box<Runnable>>>>,
    left_lower_limit: Option<Box<SubsystemFactory<Box<Runnable>>>>,
    dumper_upper_limit: Option<Box<SubsystemFactory<Box<Runnable>>>>,
    dumper_lower_limit: Option<Box<SubsystemFactory<Box<Runnable>>>>,
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
            right_upper_limit: None,
            right_lower_limit: None,
            left_upper_limit: None,
            left_lower_limit: None,
            dumper_upper_limit: None,
            dumper_lower_limit: None,
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

    pub fn with_test_intake_limits(&mut self, left_upper: Box<DigitalInput>, left_lower: Box<DigitalInput>,
                                   right_upper: Box<DigitalInput>, right_lower: Box<DigitalInput>) -> &mut Self {
        self.right_upper_limit = self.make_test_limit(self.state.get_intake().get_right_actuator().get_upper().clone(), left_upper);
        self.right_lower_limit = self.make_test_limit(self.state.get_intake().get_right_actuator().get_lower().clone(), left_lower);
        self.left_upper_limit = self.make_test_limit(self.state.get_intake().get_left_actuator().get_upper().clone(), right_upper);
        self.left_lower_limit = self.make_test_limit(self.state.get_intake().get_left_actuator().get_lower().clone(), right_lower);

        self
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
        self.intake = Box::new(ProductionIntakeFactory::new(
            self.state.clone(), self.io.clone()));
        self.left_upper_limit = self.make_production_limit(self.state.get_intake().get_left_actuator().get_upper().clone(), LEFT_UPPER_ACTUATOR_LIMIT);
        self.right_upper_limit = self.make_production_limit(self.state.get_intake().get_right_actuator().get_upper().clone(), RIGHT_UPPER_ACTUATOR_LIMIT);
        self.left_lower_limit = self.make_production_limit(self.state.get_intake().get_left_actuator().get_lower().clone(), LEFT_LOWER_ACTUATOR_LIMIT);
        self.right_lower_limit = self.make_production_limit(self.state.get_intake().get_right_actuator().get_lower().clone(), RIGHT_LOWER_ACTUATOR_LIMIT);
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
        let mut monitor = CompositeRunnable::new();

        if let Some(sensor) = self.dumper_lower_limit {
            monitor.add_runnable(sensor.produce());
        }

        if let Some(sensor) = self.dumper_upper_limit {
            monitor.add_runnable(sensor.produce());
        }

        if let Some(sensor) = self.left_lower_limit {
            monitor.add_runnable(sensor.produce());
        }

        if let Some(sensor) = self.left_upper_limit {
            monitor.add_runnable(sensor.produce());
        }

        if let Some(sensor) = self.right_lower_limit {
            monitor.add_runnable(sensor.produce());
        }

        if let Some(sensor) = self.right_upper_limit {
            monitor.add_runnable(sensor.produce());
        }

        RobotAssembler::new(dumper, drive, intake, self.state, self.bench, monitor)
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

    fn make_production_limit(&self, state: Arc<AtomicBool>, pin: Pin) -> Option<Box<SubsystemFactory<Box<Runnable>>>> {
        Some(Box::new(DigitalMonitorFactory::new(state,
                                                 self.io.generate_digital_input(pin))))
    }

    fn make_test_limit(&self, state: Arc<AtomicBool>, input: Box<DigitalInput>) -> Option<Box<SubsystemFactory<Box<Runnable>>>> {
        Some(Box::new(DigitalMonitorFactory::new(state, input)))
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