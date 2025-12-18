use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_dc_exec_simple_command() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ex", "app", "--", "ls"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec app ls"));
}

#[test]
fn test_dc_exec_command_with_args() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ex", "app", "--", "ls", "-la"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec app ls -la"));
}

#[test]
fn test_dc_exec_bash_command() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ex", "my_service", "--", "bash", "-c", "echo hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "exec my_service bash -c echo hello",
        ));
}

#[test]
fn test_dc_exec_without_service() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ex"])
        .assert()
        .success();
}

#[test]
fn test_dc_exec_with_detach_flag() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ex", "app", "-d", "--", "ls"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec -d app ls"));
}

#[test]
fn test_dc_exec_with_detach_flag_long_form() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ex", "app", "--detach", "--", "ls"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec -d app ls"));
}

#[test]
fn test_dc_exec_with_detach_and_multiple_args() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ex", "app", "-d", "--", "ls", "-la"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec -d app ls -la"));
}

#[test]
fn test_dc_exec_with_detach_and_complex_command() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&[
            "dc",
            "ex",
            "my_service",
            "-d",
            "--",
            "bash",
            "-c",
            "echo hello",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "exec -d my_service bash -c echo hello",
        ));
}
