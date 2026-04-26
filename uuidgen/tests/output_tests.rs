use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

fn get_expected_output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ExpectedOutput")
}

fn load_expected_output(filename: &str) -> String {
    let path = get_expected_output_dir().join(format!("{}.example", filename));
    fs::read_to_string(&path).expect(&format!("Failed to read expected output file: {:?}", path))
}

fn validate_guid_format(
    output: &str,
    expected_version: &str,
    uppercase: bool,
    non_hyphenated: bool,
) {
    let output = output.trim();
    let pattern = if non_hyphenated {
        if uppercase {
            format!(r"^[0-9A-F]{{32}}$",)
        } else {
            r"^[0-9a-f]{32}$".to_string()
        }
    } else {
        let version_digit = match expected_version {
            "v4" => "4",
            "v6" => "6",
            "v7" => "7",
            _ => "4",
        };
        if uppercase {
            format!(
                r"^[0-9A-F]{{8}}-[0-9A-F]{{4}}-[{}][0-9A-F]{{3}}-[89AB][0-9A-F]{{3}}-[0-9A-F]{{12}}$",
                version_digit
            )
        } else {
            format!(
                r"^[0-9a-f]{{8}}-[0-9a-f]{{4}}-[{}][0-9a-f]{{3}}-[89ab][0-9a-f]{{3}}-[0-9a-f]{{12}}$",
                version_digit
            )
        }
    };

    let re = regex::Regex::new(&pattern).unwrap();
    assert!(
        re.is_match(output),
        "Output '{}' does not match expected format pattern '{}'",
        output,
        pattern
    );
}

fn validate_nanoid_format(output: &str, count: usize) {
    let lines: Vec<&str> = output.trim().lines().collect();
    assert_eq!(
        lines.len(),
        count,
        "Expected {} nanoids, got {}",
        count,
        lines.len()
    );

    for line in lines {
        assert!(
            line.chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-'),
            "Nanoid '{}' contains invalid characters",
            line
        );
        assert_eq!(line.len(), 21, "Nanoid '{}' has incorrect length", line);
    }
}

#[test]
fn test_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd.arg("-?").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();

    // Verify help output contains key sections
    assert!(
        actual.contains("uuidgen v0.1.0-dev"),
        "Help output missing version info"
    );
    assert!(
        actual.contains("Usage:") && actual.contains("[OPTIONS]"),
        "Help output missing usage section"
    );
    assert!(
        actual.contains("--count") && actual.contains("--uuid-type"),
        "Help output missing expected options"
    );
    assert!(
        actual.contains("Examples:"),
        "Help output missing examples section"
    );
}

#[test]
fn test_default_type_guid_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd.arg("-t").arg("guid").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v4", false, false);
}

#[test]
fn test_5_instance_default_type_guid_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-c")
        .arg("5")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();

    let lines: Vec<&str> = actual.trim().lines().collect();
    assert_eq!(lines.len(), 5, "Expected 5 GUIDs, got {}", lines.len());
    for line in lines {
        validate_guid_format(line, "v4", false, false);
    }
}

#[test]
fn test_default_type_guid_hyphenated_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd.arg("-t").arg("guid").arg("-y").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v4", false, true);
}

#[test]
fn test_default_type_guid_uppercase_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd.arg("-t").arg("guid").arg("-u").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v4", true, false);
}

#[test]
fn test_default_type_guid_hyphenated_uppercase_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-y")
        .arg("-u")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v4", true, true);
}

#[test]
fn test_v6_guid_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v6")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v6", false, false);
}

#[test]
fn test_v6_guid_hyphenated_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v6")
        .arg("-y")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v6", false, true);
}

#[test]
fn test_v6_guid_uppercase_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v6")
        .arg("-u")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v6", true, false);
}

#[test]
fn test_v6_guid_hyphenated_uppercase_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v6")
        .arg("-y")
        .arg("-u")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v6", true, true);
}

#[test]
fn test_v7_guid_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v7")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v7", false, false);
}

#[test]
fn test_v7_guid_hyphenated_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v7")
        .arg("-y")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v7", false, true);
}

#[test]
fn test_v7_guid_uppercase_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v7")
        .arg("-u")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v7", true, false);
}

#[test]
fn test_v7_guid_hyphenated_uppercase_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("guid")
        .arg("-v")
        .arg("v7")
        .arg("-y")
        .arg("-u")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_guid_format(&actual, "v7", true, true);
}

#[test]
fn test_nanoid_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd.arg("-t").arg("nanoid").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_nanoid_format(&actual, 1);
}

#[test]
fn test_5_instance_nanoid_produces_expected_output() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd
        .arg("-t")
        .arg("nanoid")
        .arg("-c")
        .arg("5")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    validate_nanoid_format(&actual, 5);
}
