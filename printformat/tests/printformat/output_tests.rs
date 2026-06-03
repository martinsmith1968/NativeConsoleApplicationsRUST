use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

fn get_expected_output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ExpectedOutput")
}

fn normalize_output(s: String) -> String {
    let s = s.replace("\r\n", "\n");
    // Strip trailing whitespace from each line for robust comparison
    let trimmed: Vec<&str> = s.lines().map(|line| line.trim_end()).collect();
    let result = trimmed.join("\n");
    if s.ends_with('\n') { result + "\n" } else { result }
}

fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn current_year() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let year = 1970u64 + (secs as f64 / (365.2425 * 86400.0)) as u64;
    year.to_string()
}

fn load_expected_output(filename: &str) -> String {
    let path = get_expected_output_dir().join(format!("{}.example", filename));
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read expected output file: {:?}", path));

    let normalized = normalize_output(content)
        .replace("%APP_VERSION%", app_version())
        .replace("%CURRENT_YEAR%", &current_year());

    let bytes = normalized.as_bytes().to_vec();
    let mut result = String::new();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] > 127 {
            while i < bytes.len() && bytes[i] > 127 {
                i += 1;
            }
            result.push('©');
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

#[test]
fn execute_with_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("--help").env("COLUMNS", "500").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_help_request_produces_arguments_list");

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_with_format_and_one_arg_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("Hello, {}!").arg("World").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_format_and_one_arg_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Single-argument output does not match expected"
    );
}

#[test]
fn execute_with_format_and_two_args_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd
        .arg("{} + {} = {}")
        .arg("1")
        .arg("2")
        .arg("3")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected =
        load_expected_output("Execute_with_format_and_two_args_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Three-argument output does not match expected"
    );
}

#[test]
fn execute_with_format_and_no_args_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("Hello World").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_format_and_no_args_produces_expected_output");

    assert_eq!(
        actual, expected,
        "No-argument output does not match expected"
    );
}

#[test]
fn execute_with_right_align_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("{:>10}").arg("hello").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_right_align_produces_expected_output");

    assert_eq!(actual, expected, "Right-aligned output does not match expected");
}

#[test]
fn execute_with_fill_char_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("{:*^11}").arg("hello").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output("Execute_with_fill_char_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Fill-character output does not match expected"
    );
}
