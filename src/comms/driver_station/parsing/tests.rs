use super::*;

#[test]
fn test_drive_parsing() {
    let valid_f_f = parse_message("drive 1.0 -1.0").unwrap();
    let valid_i_i = parse_message("drive 1 -1").unwrap();
    let valid_i_f = parse_message("drive 1 -1.0").unwrap();
    let valid_f_i = parse_message("drive 1.0 -1").unwrap();

    let invalid = parse_message("drive 2 3 4");
    let invalid_bad_left = parse_message("drive hi 4");
    let invalid_bad_right = parse_message("drive 5 hi");
    let invalid_bad_args = parse_message("drive hi bye");
    let invalid_right_too_high = parse_message("drive train 1 2");
    let invalid_left_too_high = parse_message("drive train 2 1");
    let invalid_both_too_high = parse_message("drive train 2 2");


    let expected = ReceivableMessage::Drive(1.0, -1.0);

    assert_eq!(valid_f_f, expected);
    assert_eq!(valid_i_i, expected);
    assert_eq!(valid_i_f, expected);
    assert_eq!(valid_f_i, expected);
    assert!(invalid.is_err());
    assert!(invalid_bad_left.is_err());
    assert!(invalid_bad_right.is_err());
    assert!(invalid_bad_args.is_err());
    assert!(invalid_left_too_high.is_err());
    assert!(invalid_right_too_high.is_err());
    assert!(invalid_both_too_high.is_err());
}

#[test]
fn test_kill() {
    let valid = parse_message("kill").unwrap();
    let invalid = parse_message("kill hi");
    let expected_valid = ReceivableMessage::Kill;

    assert_eq!(valid, expected_valid);
    assert!(invalid.is_err());
}

#[test]
fn test_revive() {
    let valid = parse_message("revive").unwrap();
    let invalid = parse_message("revive hi");
    let expected_valid = ReceivableMessage::Revive;

    assert_eq!(valid, expected_valid);
    assert!(invalid.is_err());
}

#[test]
fn test_brake() {
    let valid = parse_message("brake").unwrap();
    let invalid = parse_message("brake hi");
    let expected_valid = ReceivableMessage::Brake;

    assert_eq!(valid, expected_valid);
    assert!(invalid.is_err());
}

#[test]
fn test_enable() {
    let valid = parse_message("enable drive_train").unwrap();
    let invalid_no_args = parse_message("enable");
    let invalid_too_many_args = parse_message("enable drive_train hi");
    let invalid_bad_subsystem = parse_message("enable fail");
    let expected_valid = ReceivableMessage::Enable(Subsystem::DriveTrain);

    assert_eq!(valid, expected_valid);
    assert!(invalid_no_args.is_err());
    assert!(invalid_too_many_args.is_err());
    assert!(invalid_bad_subsystem.is_err());
}

#[test]
fn test_disable() {
    let valid = parse_message("disable drive_train").unwrap();
    let invalid_no_args = parse_message("disable");
    let invalid_too_many_args = parse_message("disable drive_train hi");
    let invalid_bad_subsystem = parse_message("disable fail");
    let expected_valid = ReceivableMessage::Disable(Subsystem::DriveTrain);

    assert_eq!(valid, expected_valid);
    assert!(invalid_no_args.is_err());
    assert!(invalid_too_many_args.is_err());
    assert!(invalid_bad_subsystem.is_err());
}

#[test]
fn test_empty_message() {
    let actual = parse_message("").unwrap_err();
    let expected = LogData::warning("Empty message in DS Comms!");

    assert_eq!(actual.get_severity(), expected.get_severity());
    assert_eq!(actual.get_description(), expected.get_description());
}

#[test]
fn test_nonexistent_command() {
    let actual_1 = parse_message("annihilate").unwrap_err();
    let actual_2 = parse_message("annihilate Noah").unwrap_err();

    let expected_1 = LogData::warning("Received nonexistent command, message is 'annihilate'");
    let expected_2 = LogData::warning("Received nonexistent command, message is 'annihilate Noah'");

    assert_eq!(actual_1.get_severity(), expected_1.get_severity());
    assert_eq!(actual_1.get_description(), expected_1.get_description());

    assert_eq!(actual_2.get_severity(), expected_2.get_severity());
    assert_eq!(actual_2.get_description(), expected_2.get_description());
}

#[test]
fn test_removed_newline() {
    let actual = parse_message("annihilate Noah\n").unwrap_err();
    let expected = LogData::warning("Received nonexistent command, message is 'annihilate Noah'");

    assert_eq!(actual.get_severity(), expected.get_severity());
    assert_eq!(actual.get_description(), expected.get_description());
}
