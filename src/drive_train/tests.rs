use std::sync::mpsc::channel;
use std::thread::spawn;
use std::time::Duration;

use crate::devices::motor_controllers::test_motor::TestAction;
use crate::devices::motor_controllers::test_motor::TestMotor;

use super::*;
use std::thread::sleep;

const TIMEOUT: u64 = 100;

fn assert_action(command: DriveTrainCommand, left_result: TestAction, right_result: TestAction,
                 left: &Receiver<TestAction>, right: &Receiver<TestAction>,
                 command_sender: &Sender<DriveTrainCommand>) {
    command_sender.send(command.clone()).expect("Drive Train Died!");

    assert_eq!(left_result, left.recv_timeout(Duration::from_millis(TIMEOUT)).expect("Drive Train Died!"));
    assert_eq!(right_result, right.recv_timeout(Duration::from_millis(TIMEOUT)).expect("Drive Train Died!"));
}

fn assert_drive(left_speed: f32, right_speed: f32, sender: &Sender<DriveTrainCommand>,
                left: &Receiver<TestAction>, right: &Receiver<TestAction>) {
    assert_action(DriveTrainCommand::Drive(left_speed, right_speed),
                  TestAction::SetSpeed(left_speed), TestAction::SetSpeed(right_speed),
                  left, right, sender);
}

fn assert_test(test: TestAction, receiver: &Receiver<TestAction>) {
    assert_eq!(test, receiver.recv_timeout(Duration::from_millis(TIMEOUT)).expect("Drive Train Died!"));
}

#[test]
fn test_drive() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, _) = channel();
    let (command_sender, command_receiver) = channel();
    let log_sender = LogSender::new(log_sender);

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    spawn(move || {
        drive_train.start();
    });

    // test both forward
    assert_drive(1.0, 1.0, &command_sender,
                 &left_test_receiver, &right_test_receiver);

    // test both backward
    assert_drive(-1.0, -1.0, &command_sender,
                 &left_test_receiver, &right_test_receiver);

    // test left forward right back
    assert_drive(1.0, -1.0, &command_sender,
                 &left_test_receiver, &right_test_receiver);

    // test right forward left back
    assert_drive(-1.0, 1.0, &command_sender,
                 &left_test_receiver, &right_test_receiver);
}

#[test]
fn test_enable() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, _) = channel();
    let log_sender = LogSender::new(log_sender);
    let (command_sender, command_receiver) = channel();

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    spawn(move || {
        drive_train.start();
    });

    // test disable
    command_sender.send(DriveTrainCommand::Disable).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // try and send something when disabled
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // test enable
    command_sender.send(DriveTrainCommand::Enable).expect("Drive Train Died!");
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).expect("Drive Train Died!");
    assert_test(TestAction::SetSpeed(1.0), &left_test_receiver);
    assert_test(TestAction::SetSpeed(1.0), &right_test_receiver);
}

#[test]
fn test_kill() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, _) = channel();
    let (command_sender, command_receiver) = channel();
    let log_sender = LogSender::new(log_sender);

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    spawn(move || {
        drive_train.start();
    });

    // test kill
    command_sender.send(DriveTrainCommand::Kill).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // try and send something when killed
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // test revive
    command_sender.send(DriveTrainCommand::Revive).expect("Drive Train Died!");
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).expect("Drive Train Died!");
    assert_test(TestAction::SetSpeed(1.0), &left_test_receiver);
    assert_test(TestAction::SetSpeed(1.0), &right_test_receiver);
}

#[test]
fn test_interactions() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, _) = channel();
    let (command_sender, command_receiver) = channel();
    let log_sender = LogSender::new(log_sender);

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    spawn(move || {
        drive_train.start();
    });

    // test kill
    command_sender.send(DriveTrainCommand::Kill).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // test disable
    command_sender.send(DriveTrainCommand::Disable).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // try and send something when killed and disabled
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // test revive and send something when still disabled
    command_sender.send(DriveTrainCommand::Revive).expect("Drive Train Died!");
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // test enable and send something when still disabled
    command_sender.send(DriveTrainCommand::Kill).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);
    command_sender.send(DriveTrainCommand::Enable).expect("Drive Train Died!");
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);

    // test sending something when alive again
    command_sender.send(DriveTrainCommand::Revive).expect("Drive Train Died!");
    command_sender.send(DriveTrainCommand::Enable).expect("Drive Train Died!");
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).expect("Drive Train Died!");
    assert_test(TestAction::SetSpeed(1.0), &left_test_receiver);
    assert_test(TestAction::SetSpeed(1.0), &right_test_receiver);
}

#[test]
fn test_stop() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, _) = channel();
    let (command_sender, command_receiver) = channel();
    let log_sender = LogSender::new(log_sender);

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    let t = spawn(move || {
        drive_train.start();
    });

    command_sender.send(DriveTrainCommand::Stop).expect("Drive Train Died!");
    assert_test(TestAction::Stop, &left_test_receiver);
    assert_test(TestAction::Stop, &right_test_receiver);
}