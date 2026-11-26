use assert_cmd::Command;
use test_support::{GUID_FILTERS, NANOID_FILTERS, assert_output_snapshot, normalize_output};

fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Run uuidgen with the given arguments and return its normalised stdout.
fn run_uuidgen(args: &[&str]) -> String {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd.args(args).env("COLUMNS", "500").output().unwrap();
    normalize_output(String::from_utf8(output.stdout).unwrap())
}

#[test]
fn test_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("uuidgen").unwrap();
    let output = cmd.arg("-?").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();

    // Verify help output contains key sections
    assert!(
        actual.contains(&format!("uuidgen v{}", app_version())),
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
fn execute_app_with_help_request_produces_arguments_list() {
    assert_output_snapshot!(run_uuidgen(&["-?"]));
}

#[test]
fn execute_app_with_full_help_request_produces_arguments_list() {
    assert_output_snapshot!(run_uuidgen(&["--help"]));
}

#[test]
fn execute_app_for_default_type_guid_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_5_instance_default_type_guid_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-c", "5"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_default_type_guid_no_dashes_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-d"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_default_type_guid_uppercase_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-u"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_default_type_guid_no_dashes_uppercase_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-d", "-u"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_v6_guid_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-v", "v6"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_v6_guid_no_dashes_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-v", "v6", "-d"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_v6_guid_uppercase_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-v", "v6", "-u"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_v6_guid_no_dashes_uppercase_produces_expected_output() {
    assert_output_snapshot!(
        run_uuidgen(&["-t", "guid", "-v", "v6", "-d", "-u"]),
        GUID_FILTERS
    );
}

#[test]
fn execute_app_for_v7_guid_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-v", "v7"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_v7_guid_no_dashes_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-v", "v7", "-d"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_v7_guid_uppercase_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "guid", "-v", "v7", "-u"]), GUID_FILTERS);
}

#[test]
fn execute_app_for_v7_guid_no_dashes_uppercase_produces_expected_output() {
    assert_output_snapshot!(
        run_uuidgen(&["-t", "guid", "-v", "v7", "-d", "-u"]),
        GUID_FILTERS
    );
}

#[test]
fn execute_app_for_nanoid_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "nanoid"]), NANOID_FILTERS);
}

#[test]
fn execute_app_for_5_instance_nanoid_produces_expected_output() {
    assert_output_snapshot!(run_uuidgen(&["-t", "nanoid", "-c", "5"]), NANOID_FILTERS);
}
