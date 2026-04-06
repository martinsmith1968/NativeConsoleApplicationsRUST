use assert_cmd::Command;
use predicates::prelude::*;

// ===== CLI Help and Version Tests =====

#[test]
fn test_cli_help_flag() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate Unique IDs"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version_flag() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("uuidgen"));
}

// ===== UUID V4 CLI Tests =====

#[test]
fn test_cli_v4_default_generation() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$").unwrap());
}

#[test]
fn test_cli_v4_uppercase() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--uppercase")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}\n$").unwrap());
}

#[test]
fn test_cli_v4_non_hyphenated() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--non-hyphenated")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{32}\n$").unwrap());
}

#[test]
fn test_cli_v4_uppercase_non_hyphenated() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--uppercase")
        .arg("--non-hyphenated")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9A-F]{32}\n$").unwrap());
}

// ===== UUID V6 CLI Tests =====

#[test]
fn test_cli_v6_with_default_seed() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--guid-version")
        .arg("v6")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{8}-[0-9a-f]{4}-6[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$").unwrap());
}

#[test]
fn test_cli_v6_with_custom_seed() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--guid-version")
        .arg("v6")
        .arg("--guid-v6-seed")
        .arg("100,101,102,103,104,105")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{8}-[0-9a-f]{4}-6[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$").unwrap());
}

#[test]
fn test_cli_v6_with_invalid_seed_shows_warning() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--guid-version")
        .arg("v6")
        .arg("--guid-v6-seed")
        .arg("abc,def,ghi,jkl,mno,pqr")
        .assert()
        .success()
        .stderr(predicate::str::contains("Warning: unable to use seed values"));
}

// ===== UUID V7 CLI Tests =====

#[test]
fn test_cli_v7_generation() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--guid-version")
        .arg("v7")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$").unwrap());
}

// ===== NanoID CLI Tests =====

#[test]
fn test_cli_nanoid_default() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--uuid-type")
        .arg("nanoid")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[A-Za-z0-9_-]{21}\n$").unwrap());
}

#[test]
fn test_cli_nanoid_custom_length() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--uuid-type")
        .arg("nanoid")
        .arg("--nanoid-length")
        .arg("10")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[A-Za-z0-9_-]{10}\n$").unwrap());
}

#[test]
fn test_cli_nanoid_length_100() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--uuid-type")
        .arg("nanoid")
        .arg("--nanoid-length")
        .arg("100")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[A-Za-z0-9_-]{100}\n$").unwrap());
}

// ===== Count Tests =====

#[test]
fn test_cli_count_multiple() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--count")
        .arg("5")
        .assert()
        .success();
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    
    assert_eq!(lines.len(), 5);
}

#[test]
fn test_cli_count_single() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--count")
        .arg("1")
        .assert()
        .success();
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    
    assert_eq!(lines.len(), 1);
}

// ===== Template Tests =====

#[test]
fn test_cli_template_with_uuid_placeholder() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--output-template")
        .arg("UUID:{uuid}")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("UUID:"));
}

#[test]
fn test_cli_template_with_sequence_placeholder() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--output-template")
        .arg("ID-{sequence}")
        .arg("--count")
        .arg("3")
        .assert()
        .success()
        .stdout(predicate::str::contains("ID-1\n"))
        .stdout(predicate::str::contains("ID-2\n"))
        .stdout(predicate::str::contains("ID-3\n"));
}

#[test]
fn test_cli_template_with_both_placeholders() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--output-template")
        .arg("{sequence}:{uuid}")
        .arg("--count")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains("1:"))
        .stdout(predicate::str::contains("2:"));
}

#[test]
fn test_cli_template_invalid_placeholder_shows_error() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--output-template")
        .arg("{invalid}")
        .assert()
        .success()
        .stderr(predicate::str::contains("Error: Invalid output template"));
}

// ===== Short Option Tests =====

#[test]
fn test_cli_short_options() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("-c")
        .arg("1")
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v4")
        .arg("-u")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9A-F]{32}\n$").unwrap());
}

// ===== Invalid Arguments Tests =====

#[test]
fn test_cli_invalid_uuid_type() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--uuid-type")
        .arg("invalid")
        .assert()
        .failure();
}

#[test]
fn test_cli_invalid_guid_version() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--guid-version")
        .arg("v99")
        .assert()
        .failure();
}

#[test]
fn test_cli_invalid_count_zero() {
    // Count of 0 should be rejected by clap or result in no output
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--count")
        .arg("0")
        .assert()
        .success(); // The loop 1..=0 produces no iterations, so success with no output
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "");
}

#[test]
fn test_cli_invalid_count_negative() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--count")
        .arg("-5")
        .assert()
        .failure();
}

// ===== Combined Options Tests =====

#[test]
fn test_cli_v6_uppercase_non_hyphenated_with_template() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("-v")
        .arg("v6")
        .arg("-u")
        .arg("-y")
        .arg("-o")
        .arg("ID:{uuid}")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("ID:"))
        .stdout(predicate::str::is_match(r"^ID:[0-9A-F]{32}\n$").unwrap());
}

#[test]
fn test_cli_nanoid_with_template_and_count() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("-t")
        .arg("nanoid")
        .arg("-l")
        .arg("15")
        .arg("-o")
        .arg("NID-{sequence}-{uuid}")
        .arg("-c")
        .arg("3")
        .assert()
        .success()
        .stdout(predicate::str::contains("NID-1-"))
        .stdout(predicate::str::contains("NID-2-"))
        .stdout(predicate::str::contains("NID-3-"));
}

// ===== Exit Code Tests =====

#[test]
fn test_cli_success_exit_code() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.assert()
        .success();
}

#[test]
fn test_cli_help_exit_code() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--help")
        .assert()
        .code(0);
}

#[test]
fn test_cli_version_exit_code() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--version")
        .assert()
        .code(0);
}

#[test]
fn test_cli_invalid_arg_exit_code() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    cmd.arg("--invalid-option")
        .assert()
        .failure();
}
