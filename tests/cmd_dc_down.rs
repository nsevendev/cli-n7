use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_dc_down_cmd() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "d"])
        .assert()
        .success()
        .stdout(predicate::str::contains("down"));
}

#[test]
fn test_dc_down_cmd_with_rm_volume() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "d", "-v"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-v"));
}

#[test]
fn test_dc_down_cmd_with_rm_orphan() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "d", "-o"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--remove-orphans"));
}

#[test]
fn test_dc_down_cmd_with_all_args() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "d", "-v", "-o"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-v --remove-orphans"));
}
