use std::thread::spawn;
use std::time::Duration;

use crate::devices::motor_controllers::test_motor::TestAction;
use crate::devices::motor_controllers::test_motor::TestMotor;

use super::*;
use std::sync::mpsc::channel;

const TIMEOUT: u64 = 100;

#[test]
fn test_drive() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, log_receiver) = channel();
    let (command_sender, command_receiver) = channel();

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    spawn(move || {
        drive_train.start();
    });

    // test both forward
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), right_result);

    // test both backward
    command_sender.send(DriveTrainCommand::Drive(-1.0, -1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(-1.0), left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(-1.0), right_result);

    // test left forward right back
    command_sender.send(DriveTrainCommand::Drive(1.0, -1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(-1.0), right_result);

    // test right forward left back
    command_sender.send(DriveTrainCommand::Drive(-1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(-1.0), left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), right_result);
}

#[test]
fn test_enable() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, log_receiver) = channel();
    let (command_sender, command_receiver) = channel();

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    spawn(move || {
        drive_train.start();
    });

    // test disable
    command_sender.send(DriveTrainCommand::Disable).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, right_result);

    // try and send something when disabled
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, right_result);

    // test enable
    command_sender.send(DriveTrainCommand::Enable).unwrap();

    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), right_result);
}

#[test]
fn test_kill() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, log_receiver) = channel();
    let (command_sender, command_receiver) = channel();

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    spawn(move || {
        drive_train.start();
    });

    // test disable
    command_sender.send(DriveTrainCommand::Kill).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, right_result);

    // try and send something when disabled
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, right_result);

    // test enable
    command_sender.send(DriveTrainCommand::Revive).unwrap();

    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), right_result);
}

#[test]
fn test_interactions() {
    let (left_test_sender, left_test_receiver) = channel();
    let (right_test_sender, right_test_receiver) = channel();
    let (log_sender, log_receiver) = channel();
    let (command_sender, command_receiver) = channel();

    let left_motor = TestMotor::new(left_test_sender);
    let right_motor = TestMotor::new(right_test_sender);

    let mut drive_train = DriveTrain::new(command_receiver, log_sender, Box::new(left_motor), Box::new(right_motor));

    spawn(move || {
        drive_train.start();
    });

    // test disable
    command_sender.send(DriveTrainCommand::Kill).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, right_result);

    // test disable
    command_sender.send(DriveTrainCommand::Disable).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, right_result);

    // try and send something when disabled
    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, right_result);

    // test enable
    command_sender.send(DriveTrainCommand::Revive).unwrap();

    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::Stop, right_result);

    // test enable
    command_sender.send(DriveTrainCommand::Enable).unwrap();

    command_sender.send(DriveTrainCommand::Drive(1.0, 1.0)).unwrap();

    let left_result = left_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), left_result);

    let right_result = right_test_receiver.recv_timeout(Duration::from_millis(TIMEOUT)).unwrap();
    assert_eq!(TestAction::SetSpeed(1.0), right_result);
}