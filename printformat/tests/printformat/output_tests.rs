use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

// From : https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
macro_rules! get_current_function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

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
    if s.ends_with('\n') {
        result + "\n"
    } else {
        result
    }
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
fn execute_app_with_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("-?").env("COLUMNS", "500").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output(&get_current_function_name!());

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_app_with_full_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("--help").env("COLUMNS", "500").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output(&get_current_function_name!());

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_app_with_text_only_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd.arg("bob").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output(&get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Single-argument output does not match expected"
    );
}

#[test]
fn execute_app_with_2_placeholders_and_string_values_produces_expected_output() {
    let mut cmd = Command::cargo_bin("printformat").unwrap();
    let output = cmd
        .arg("{} {}")
        .arg("Hello")
        .arg("World")
        .output()
        .unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Fill-character output does not match expected"
    );
}
