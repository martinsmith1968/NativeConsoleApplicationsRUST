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

#[test]
fn test_cli_writes_to_closed_pipe_without_panicking() {
    use std::process::{Command as StdCommand, Stdio};

    let mut child = StdCommand::new(env!("CARGO_BIN_EXE_printformat"))
        .arg("Hello World")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start printformat");

    // Close the read end of the pipe before the child can write.
    drop(child.stdout.take());

    let output = child
        .wait_with_output()
        .expect("failed to wait for printformat");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "expected success on a closed pipe, got {:?}: {}",
        output.status,
        stderr
    );
    assert!(!stderr.contains("panicked"), "stderr was: {}", stderr);
}

// ===== Alignment and Padding Tests =====

#[test]
fn test_cli_right_align() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{:>10}")
        .arg("hello")
        .assert()
        .success()
        .stdout("     hello\n");
}

#[test]
fn test_cli_left_align() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{:<10}")
        .arg("hello")
        .assert()
        .success()
        .stdout("hello     \n");
}

#[test]
fn test_cli_center_align() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{:^11}")
        .arg("hello")
        .assert()
        .success()
        .stdout("   hello   \n");
}

#[test]
fn test_cli_fill_char_center() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{:*^11}")
        .arg("hello")
        .assert()
        .success()
        .stdout("***hello***\n");
}

#[test]
fn test_cli_fill_char_right() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{:0>5}")
        .arg("42")
        .assert()
        .success()
        .stdout("00042\n");
}

#[test]
fn test_cli_fill_char_left() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{:-<8}")
        .arg("hi")
        .assert()
        .success()
        .stdout("hi------\n");
}

#[test]
fn test_cli_width_only() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("{:10}").arg("hi").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(
        stdout.len(),
        11,
        "stdout should be 10 characters plus newline"
    );
    assert!(
        stdout.contains("hi"),
        "stdout should contain the formatted value"
    );
}

#[test]
fn test_cli_multiple_aligned() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{:>5} | {:<5}")
        .arg("a")
        .arg("b")
        .assert()
        .success()
        .stdout("    a | b    \n");
}

#[test]
fn test_cli_escaped_open_brace() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{{}}").assert().success().stdout("{}\n");
}

#[test]
fn test_cli_escaped_braces_with_arg() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("{{}} and {}")
        .arg("value")
        .assert()
        .success()
        .stdout("{} and value\n");
}

#[test]
fn test_cli_align_with_mixed() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("Name: {:<10} Age: {:>3}")
        .arg("Alice")
        .arg("30")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice     "))
        .stdout(predicate::str::contains(" 30"))
        .stdout("Name: Alice      Age:  30\n");
}

#[test]
fn test_cli_csharp_basic_format() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--format-type")
        .arg("CSharp")
        .arg("{0} is {1} years old")
        .arg("Alice")
        .arg("30")
        .assert()
        .success()
        .stdout("Alice is 30 years old\n");
}

#[test]
fn test_cli_csharp_alignment() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--format-type")
        .arg("CSharp")
        .arg("{0,-10} | {1,10}")
        .arg("left")
        .arg("right")
        .assert()
        .success()
        .stdout("left       |      right\n");
}

#[test]
fn test_cli_csharp_zero_padding() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--format-type")
        .arg("CSharp")
        .arg("{0:D5}")
        .arg("42")
        .assert()
        .success()
        .stdout("00042\n");
}

#[test]
fn test_cli_csharp_hex() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--format-type")
        .arg("CSharp")
        .arg("{0:X4}")
        .arg("255")
        .assert()
        .success()
        .stdout("00FF\n");
}

#[test]
fn test_cli_csharp_number_and_fixed_point() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--format-type")
        .arg("CSharp")
        .arg("{0:N2} | {1:F3}")
        .arg("1234567.891")
        .arg("3.14159")
        .assert()
        .success()
        .stdout("1,234,567.89 | 3.142\n");
}

#[test]
fn test_cli_csharp_currency_default_symbol() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--format-type")
        .arg("CSharp")
        .arg("{0:C}")
        .arg("1234.5")
        .assert()
        .success()
        .stdout("£1,234.50\n");
}

#[test]
fn test_cli_csharp_currency_option() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--format-type")
        .arg("CSharp")
        .arg("--currency")
        .arg("€")
        .arg("--")
        .arg("{0:C0}")
        .arg("-1234.5")
        .assert()
        .success()
        .stdout("-€1,235\n");
}

#[test]
fn test_cli_csharp_non_numeric_argument_fails() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    cmd.arg("--format-type")
        .arg("CSharp")
        .arg("{0:F2}")
        .arg("abc")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("not a valid number"));
}

// ===== C Format Tests =====

#[test]
fn test_c_format_string_substitution() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%s")
        .arg("hello")
        .assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn test_c_format_integer() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%d")
        .arg("42")
        .assert()
        .success()
        .stdout("42\n");
}

#[test]
fn test_c_format_zero_padded_integer() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%05d")
        .arg("42")
        .assert()
        .success()
        .stdout("00042\n");
}

#[test]
fn test_c_format_float_precision() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%.2f")
        .arg("3.14159")
        .assert()
        .success()
        .stdout("3.14\n");
}

#[test]
fn test_c_format_right_aligned_string() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%10s")
        .arg("hi")
        .assert()
        .success()
        .stdout("        hi\n");
}

#[test]
fn test_c_format_left_aligned_string() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%-10s")
        .arg("hi")
        .assert()
        .success()
        .stdout("hi        \n");
}

#[test]
fn test_c_format_hex_lowercase() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%x")
        .arg("255")
        .assert()
        .success()
        .stdout("ff\n");
}

#[test]
fn test_c_format_hex_uppercase() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%X")
        .arg("255")
        .assert()
        .success()
        .stdout("FF\n");
}

#[test]
fn test_c_format_escaped_percent() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("100%%")
        .assert()
        .success()
        .stdout("100%\n");
}

#[test]
fn test_c_format_multiple_args() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%s is %d years old")
        .arg("Alice")
        .arg("30")
        .assert()
        .success()
        .stdout("Alice is 30 years old\n");
}

#[test]
fn test_c_format_too_few_args_fails() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%d %d")
        .arg("1")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("placeholder"));
}

#[test]
fn test_c_format_invalid_integer_fails() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%d")
        .arg("notanumber")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("integer"));
}

#[test]
fn test_c_format_char_with_multiple_characters_fails() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%c")
        .arg("Hello")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("must be a single character"));
}

#[test]
fn test_c_format_unsupported_n_specifier_fails() {
    Command::cargo_bin("printformat")
        .unwrap()
        .arg("--format-type")
        .arg("C")
        .arg("%n")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("not supported"));
}
