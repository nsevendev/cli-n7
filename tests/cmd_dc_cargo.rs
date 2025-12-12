use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_dc_cargo_custom_command() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "c", "my_service", "--", "build", "--release"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "exec my_service cargo build --release",
        ));
}

#[test]
fn test_dc_cargo_test_without_args() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ct", "my_service"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec my_service cargo test"));
}

#[test]
fn test_dc_cargo_test_with_args() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "ct", "my_service", "--", "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "exec my_service cargo test --verbose",
        ));
}

#[test]
fn test_dc_cargo_fmt_without_args() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "cf", "app"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec app cargo fmt"));
}

#[test]
fn test_dc_cargo_fmt_with_check() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "cf", "app", "--", "--check"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec app cargo fmt --check"));
}

#[test]
fn test_dc_cargo_clippy_without_args() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "cc", "app"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec app cargo clippy"));
}

#[test]
fn test_dc_cargo_clippy_with_args() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "cc", "app", "--", "--fix"])
        .assert()
        .success()
        .stdout(predicate::str::contains("exec app cargo clippy --fix"));
}

#[test]
fn test_dc_rcheck() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "rcheck", "app"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Running rcheck: fmt -> clippy -> test",
        ))
        .stdout(predicate::str::contains("exec app cargo fmt"))
        .stdout(predicate::str::contains("exec app cargo clippy"))
        .stdout(predicate::str::contains("exec app cargo test"));
}
