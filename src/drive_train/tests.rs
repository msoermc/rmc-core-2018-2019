use std::sync::mpsc::channel;
use std::thread::spawn;
use std::time::Duration;

use crate::devices::motor_controllers::test_motor::TestAction;
use crate::devices::motor_controllers::test_motor::TestMotor;

use super::*;
use std::thread::sleep;
use std::sync::mpsc::Sender;

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

}

#[test]
fn test_enable() {

}

#[test]
fn test_kill() {

}

#[test]
fn test_stop() {

}