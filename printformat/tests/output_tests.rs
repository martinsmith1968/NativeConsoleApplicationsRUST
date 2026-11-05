use assert_cmd::Command;
use std::path::PathBuf;
use test_support::{
    get_current_function_name, load_expected_output, normalize_output_trimmed as normalize_output,
};

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
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("-?").env("COLUMNS", "500").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_app_with_full_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("--help").env("COLUMNS", "500").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_app_with_text_only_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("bob").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Single-argument output does not match expected"
    );
}

#[test]
fn execute_app_with_2_placeholders_and_string_values_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("{} {}").arg("Hello").arg("World").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Three-argument output does not match expected"
    );
}

#[test]
fn execute_app_with_2_placeholders_and_mixed_values_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd
        .arg("{} is {}")
        .arg("TATLTUAE")
        .arg("42")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "No-argument output does not match expected"
    );
}

#[test]
fn execute_app_with_left_aligned_text_parameter_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("!{:<10}!").arg("bob").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Right-aligned output does not match expected"
    );
}

#[test]
fn execute_app_with_right_aligned_text_parameter_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("!{:>10}!").arg("bob").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Fill-character output does not match expected"
    );
}

#[test]
fn execute_app_with_center_aligned_text_parameter_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("!{:^10}!").arg("bob").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Fill-character output does not match expected"
    );
}

#[test]
fn execute_app_with_center_aligned_character_padded_text_parameter_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("!{:-^10}!").arg("bob").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = expected_output(get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Fill-character output does not match expected"
    );
}
