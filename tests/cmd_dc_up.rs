use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_dc_up_cmd() {
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.args(&["dc", "u"])
        .assert()
        .success()
        .stdout(predicate::str::contains("up -d"));
}

#[test]
fn test_dc_up_with_build() {
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.args(&["dc", "u", "--build"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--build"));
}

#[test]
fn test_dc_up_with_no_detach() {
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.args(&["dc", "u", "--no-detach"])
        .assert()
        .success()
        .stdout(predicate::str::contains("up"))
        .stdout(predicate::str::contains("up"));
}

#[test]
fn test_dc_up_with_build_short_args() {
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.args(&["dc", "u", "-b"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--build"));
}

#[test]
fn test_dc_up_with_no_detach_short_args() {
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.args(&["dc", "u", "-n"])
        .assert()
        .success()
        .stdout(predicate::str::contains("up"))
        .stdout(predicate::str::contains("up"));
}
