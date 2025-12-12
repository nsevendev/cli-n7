use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_dc_shell_cmd() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "s", "my_service"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec -it my_service bash"));
}

#[test]
fn test_dc_shell_cmd_with_custom_shell() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "s", "my_service", "--shell", "sh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec -it my_service sh"));
}

#[test]
fn test_dc_shell_cmd_with_custom_shell_short_arg() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "s", "app", "-s", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec -it app zsh"));
}

#[test]
fn test_dc_shell_cmd_different_service() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "s", "database"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec -it database bash"));
}
