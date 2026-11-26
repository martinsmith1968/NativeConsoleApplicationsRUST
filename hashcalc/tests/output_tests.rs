use assert_cmd::Command;
use test_support::{assert_output_snapshot, normalize_output};

#[test]
fn execute_app_with_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("-?").env("COLUMNS", "500").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_full_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("--help").env("COLUMNS", "500").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}

#[test]
fn execute_app_with_text_only_default_algorithm_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("-t").arg("Hello World").output().unwrap();
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
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
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
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
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
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
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
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
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
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
    assert_output_snapshot!(normalize_output(String::from_utf8(output.stdout).unwrap()));
}
