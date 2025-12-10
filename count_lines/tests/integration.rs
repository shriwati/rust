use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_no_arguments() {
    // Test with no arguments - should fail with "Please provide a path to a file"
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("count_lines"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Please provide a path to a file"));
}

#[test]
fn test_with_valid_file() {
    // Test with a valid file path - should succeed and show line/word count
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("count_lines"));
    cmd.arg("src/main.rs")
        .arg("src/lib.rs")
        .assert()
        .success()
        .stdout(predicate::str::contains("lines"))
        .stdout(predicate::str::contains("words"));
}

#[test]
fn test_with_word_flag(){
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("count_lines"));
        cmd.args(&["-w", "src/main.rs"])
        .assert()
        .success()
        .stdout(predicate::str::contains("words"));

}