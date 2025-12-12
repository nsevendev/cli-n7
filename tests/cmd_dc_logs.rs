use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_dc_logs_all_services_with_follow() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "l"])
        .assert()
        .success()
        .stdout(predicate::str::contains("logs -f"));
}

#[test]
fn test_dc_logs_all_services_without_follow() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "l", "--no-follow"])
        .assert()
        .success()
        .stdout(predicate::str::contains("compose logs").and(predicate::str::contains("-f").not()));
}

#[test]
fn test_dc_logs_all_services_without_follow_short_arg() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "l", "-n"])
        .assert()
        .success()
        .stdout(predicate::str::contains("compose logs").and(predicate::str::contains("-f").not()));
}

#[test]
fn test_dc_logs_specific_service_with_follow() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "l", "my_service"])
        .assert()
        .success()
        .stdout(predicate::str::contains("logs -f my_service"));
}

#[test]
fn test_dc_logs_specific_service_without_follow() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "l", "my_service", "--no-follow"])
        .assert()
        .success()
        .stdout(predicate::str::contains("logs my_service"));
}

#[test]
fn test_dc_logs_different_service() {
    let _lock = n7::test_utils::lock_test();
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.env("N7_DRY_RUN", "1")
        .args(&["dc", "l", "database"])
        .assert()
        .success()
        .stdout(predicate::str::contains("logs -f database"));
}
