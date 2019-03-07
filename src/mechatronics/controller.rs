use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;

use crate::framework::Runnable;
use crate::mechatronics::bucket_ladder::Intake;
use crate::mechatronics::commands::RobotCommand;
use crate::mechatronics::drive_train::DriveTrain;
use crate::mechatronics::dumper::Dumper;
use crate::status::life::GlobalLifeState;
use std::thread;

pub enum MechState {
    Digging,
    Driving,
    Dumping,
}

pub struct RobotController {
    command_receiver: Receiver<Box<RobotCommand>>,
    drive_train: DriveTrain,
    dumper: Dumper,
    intake: Intake,
    life: Arc<GlobalLifeState>,
    cycles: Arc<AtomicUsize>,
}

impl Runnable for RobotController {
    fn init(&mut self) {
        info!("Initializing controller!");
        self.drive_train.disable();
        self.dumper.disable();
        self.intake.disable();
    }

    fn run(&mut self) {
        if let Ok(message) = self.command_receiver.try_recv() {
            self.handle_message(message);
        }

        self.drive_train.run_cycle();
        self.dumper.run_cycle();
        self.intake.run_cycle();

        self.cycles.fetch_add(1, Ordering::SeqCst);

        thread::yield_now();
    }
}

impl RobotController {
    pub fn new(command_receiver: Receiver<Box<RobotCommand>>, drive_train: DriveTrain,
               dumper: Dumper, intake: Intake, life: Arc<GlobalLifeState>, cycles: Arc<AtomicUsize>) -> Self {
        Self {
            command_receiver,
            drive_train,
            dumper,
            intake,
            life,
            cycles,
        }
    }

    pub fn get_drive_train(&mut self) -> &mut DriveTrain {
        &mut self.drive_train
    }

    pub fn get_dumper(&mut self) -> &mut Dumper {
        &mut self.dumper
    }

    pub fn get_intake(&mut self) -> &mut Intake {
        &mut self.intake
    }

    pub fn get_life(&mut self) -> &GlobalLifeState {
        &self.life
    }

    fn handle_message(&mut self, command: Box<RobotCommand>) {
        command.execute(self);
    }
}
