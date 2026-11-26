use assert_cmd::Command;
use test_support::{assert_output_snapshot, normalize_output_trimmed as normalize_output};

#[test]
fn execute_app_with_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("-?").env("COLUMNS", "500").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_full_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("--help").env("COLUMNS", "500").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_text_only_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("bob").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_2_placeholders_and_string_values_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("{} {}").arg("Hello").arg("World").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
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
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_left_aligned_text_parameter_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("!{:<10}!").arg("bob").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_right_aligned_text_parameter_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("!{:>10}!").arg("bob").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_center_aligned_text_parameter_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("!{:^10}!").arg("bob").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_center_aligned_character_padded_text_parameter_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("!{:-^10}!").arg("bob").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}
