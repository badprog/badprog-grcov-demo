// https://github.com/badprog/badprog-grcov-demo

// use
use assert_cmd::Command;
use p1_arg_verifier::program::utils::*;
use predicates::prelude::PredicateBooleanExt;

// ------------------------------------------------------------------------
// start
#[test]
fn test_start_success() {
    let expected_arg = "https://www.badprog.com";
    let mut cmd = Command::new(COMMAND_CARGO);

    cmd.arg("run")
        .arg("-p")
        .arg(PACKAGE_NAME)
        .arg("--quiet")
        .arg("--")
        .arg(expected_arg);

    cmd.assert()
        .success()
        .stdout(
            predicates::str::contains(ICON_SUCCESS).and(predicates::str::contains(expected_arg)),
        )
        .stderr(predicates::str::is_empty());
}

#[test]
fn test_start_error() {
    let mut cmd = Command::new(COMMAND_CARGO);

    cmd.arg("run")
        .arg("-p")
        .arg(PACKAGE_NAME)
        .arg("--quiet")
        .arg("--");

    cmd.assert()
        .success()
        .stdout(predicates::str::is_empty())
        .stderr(
            predicates::str::contains(ICON_ERROR).and(predicates::str::contains(MESSAGE_ARG_ERROR)),
        );
}
