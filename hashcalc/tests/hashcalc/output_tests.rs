use assert_cmd::Command;
use std::path::PathBuf;
use test_support::{get_current_function_name, load_expected_output, normalize_output};

fn expected_output(filename: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ExpectedOutput")
        .join(format!("{filename}.example"));

    load_expected_output(
        &path,
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_YEAR"),
        normalize_output,
    )
}

#[test]
fn execute_app_with_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("-?").env("COLUMNS", "500").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_app_with_text_only_default_algorithm_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("-t").arg("Hello World").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Text with default algorithm output does not match expected"
    );
}

#[test]
fn execute_app_with_text_only_sha256_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("sha256")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Text with SHA256 output does not match expected"
    );
}

#[test]
fn execute_app_with_text_only_sha512_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("sha512")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Text with SHA512 output does not match expected"
    );
}

#[test]
fn execute_app_with_text_only_sha1_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("sha1")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Text with SHA1 output does not match expected"
    );
}

#[test]
fn execute_app_with_text_only_md5_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("md5")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Text with MD5 output does not match expected"
    );
}

#[test]
fn execute_app_with_text_only_base64_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd
        .arg("-t")
        .arg("Hello World")
        .arg("-a")
        .arg("base64")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Text with base64 output does not match expected"
    );
}
