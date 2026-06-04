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
    s.replace("\r\n", "\n")
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

    // Convert bytes to fix encoding issues with copyright symbol
    let bytes = normalized.as_bytes().to_vec();
    let mut result = String::new();
    let mut i = 0;

    while i < bytes.len() {
        // Detect and skip malformed UTF-8 sequences
        if bytes[i] > 127 {
            // Skip non-ASCII byte sequences (often malformed copyright symbols)
            while i < bytes.len() && bytes[i] > 127 {
                i += 1;
            }
            // Replace with proper copyright symbol
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
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("-?").env("COLUMNS", "500").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output(&get_current_function_name!());

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_app_with_text_only_default_algorithm_produces_expected_output() {
    let mut cmd = Command::cargo_bin("hashcalc").unwrap();
    let output = cmd.arg("-t").arg("Hello World").output().unwrap();
    let actual = normalize_output(String::from_utf8(output.stdout).unwrap());
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

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
    let expected = load_expected_output(&get_current_function_name!());

    assert_eq!(
        actual, expected,
        "Text with base64 output does not match expected"
    );
}
