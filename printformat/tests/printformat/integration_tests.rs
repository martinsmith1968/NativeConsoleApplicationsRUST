use assert_cmd::Command;
use predicates::prelude::*;

// ===== CLI Help and Version Tests =====

#[test]
fn test_cli_help_flag() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Format and print text"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version_flag() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("printformat"));
}

#[test]
fn test_cli_help_short_alias() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("-?").assert().success();
}

#[test]
fn test_cli_version_short_alias() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("-!").assert().success();
}

// ===== Formatting Success Tests =====

#[test]
fn test_cli_format_no_args() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("Hello World")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World"));
}

#[test]
fn test_cli_format_one_arg() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("Hello, {}!")
        .arg("World")
        .assert()
        .success()
        .stdout("Hello, World!\n");
}

#[test]
fn test_cli_format_two_args() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{} + {} = {}")
        .arg("1")
        .arg("2")
        .arg("3")
        .assert()
        .success()
        .stdout("1 + 2 = 3\n");
}

#[test]
fn test_cli_format_empty_string() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("").assert().success().stdout("\n");
}

#[test]
fn test_cli_format_multiple_placeholders() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{} {} {}")
        .arg("a")
        .arg("b")
        .arg("c")
        .assert()
        .success()
        .stdout("a b c\n");
}

#[test]
fn test_cli_format_with_special_chars() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("say \"hi\"")
        .assert()
        .success()
        .stdout("say \"hi\"\n");
}

#[test]
fn test_cli_format_numbers() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("Value: {}")
        .arg("42")
        .assert()
        .success()
        .stdout("Value: 42\n");
}

#[test]
fn test_cli_format_consecutive_placeholders() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{}{}")
        .arg("foo")
        .arg("bar")
        .assert()
        .success()
        .stdout("foobar\n");
}

// ===== Formatting Error Tests =====

#[test]
fn test_cli_format_missing_args() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("Hello, {}!")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn test_cli_format_too_many_args() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("Hello")
        .arg("extra")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn test_cli_no_args_at_all() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

// ===== Exit Code Tests =====

#[test]
fn test_cli_success_exit_code() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("Hello World").assert().code(0);
}

#[test]
fn test_cli_error_exit_code() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("Hello, {}!").assert().code(1);
}
