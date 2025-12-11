use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_cmd_version() {
    let mut cmd = cargo_bin_cmd!("n7");
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("n7 v"))
        .stdout(predicate::str::contains("rust"));
}
