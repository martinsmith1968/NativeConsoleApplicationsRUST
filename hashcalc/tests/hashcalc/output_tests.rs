use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

fn get_expected_output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ExpectedOutput")
}

fn normalize_output(s: String) -> String {
    s.replace("\r\n", "\n")
}

fn load_expected_output(filename: &str) -> String {
    let path = get_expected_output_dir().join(format!("{}.example", filename));
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read expected output file: {:?}", path));
    // Normalize line endings so comparison works regardless of OS or git autocrlf settings
    let normalized = normalize_output(content);
    // Replace %ENV_VAR_NAME% tokens with actual environment variable values
    let mut result = normalized;
    for (key, value) in std::env::vars() {
        let token = format!("%{}%", key);
        result = result.replace(&token, &value);
    }
    result
}

#[test]
fn execute_with_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("-?").env("COLUMNS", "500").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_help_request_produces_arguments_list");

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_with_text_only_default_algorithm_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("-t").arg("Hello World").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_text_only_default_algorithm_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Text with default algorithm output does not match expected"
    );
}

#[test]
fn execute_with_text_only_sha256_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("sha256")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_text_only_sha256_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Text with SHA256 output does not match expected"
    );
}

#[test]
fn execute_with_text_only_sha512_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("sha512")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_text_only_sha512_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Text with SHA512 output does not match expected"
    );
}

#[test]
fn execute_with_text_only_sha1_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("sha1")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_text_only_sha1_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Text with SHA1 output does not match expected"
    );
}

#[test]
fn execute_with_text_only_md5_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("md5")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_text_only_md5_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Text with MD5 output does not match expected"
    );
}

#[test]
fn execute_with_text_only_base64_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("base64")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_text_only_base64_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Text with base64 output does not match expected"
    );
}
