use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;

fn get_expected_output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("Expectedoutput")
}

fn load_expected_output(filename: &str) -> String {
    let path = get_expected_output_dir().join(format!("{}.example", filename));
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read expected output file: {:?}", path));
    // Normalize line endings so tests pass regardless of git autocrlf settings
    content.replace("\r\n", "\n")
}

fn run_bannertext(args: &[&str]) -> String {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    for arg in args {
        cmd.arg(arg);
    }
    let output = cmd.output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

// ===== Help Output Tests =====

#[test]
fn test_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("-?").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();

    assert!(
        actual.contains("bannertext v0.1.0-dev"),
        "Help output should contain version info, got:\n{}",
        actual
    );
    assert!(
        actual.contains("Usage:"),
        "Help output should contain 'Usage:'"
    );
    assert!(
        actual.contains("--min-total-length"),
        "Help output should list '--min-total-length'"
    );
    assert!(
        actual.contains("--text-alignment"),
        "Help output should list '--text-alignment'"
    );
}

// ===== Default Banner Exact Output Tests =====

#[test]
fn test_default_banner_exact_output() {
    // natural_length = prefix_total(4) + "Hello World"(11) + suffix_total(4) = 19
    let stdout = run_bannertext(&["Hello World"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 3, "Default banner should have exactly 3 lines");
    assert_eq!(
        lines[0], "*******************",
        "Header line should be 19 '*' chars"
    );
    assert_eq!(lines[1], "**  Hello World  **", "Text line mismatch");
    assert_eq!(
        lines[2], "*******************",
        "Footer line should be 19 '*' chars"
    );
}

// ===== Min-Length Output Tests =====

#[test]
fn test_min_length_80_output() {
    let stdout = run_bannertext(&["Hello World", "--min-total-length", "80"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 3, "Should still have 3 lines");
    for (i, line) in lines.iter().enumerate() {
        assert_eq!(
            line.len(),
            80,
            "Line {} should be exactly 80 chars, got {}",
            i,
            line.len()
        );
    }
    assert!(
        lines[1].starts_with("**  Hello World"),
        "Text line should start with '**  Hello World' (left alignment default)"
    );
    assert!(
        lines[1].ends_with("  **"),
        "Text line should end with '  **'"
    );
}

// ===== Alignment Exact Output Tests =====

#[test]
fn test_center_alignment_output() {
    // text_area = 80 - prefix_total(4) - suffix_total(4) = 72
    // format!("{:^72}", "Hello World"): 72-11=61 extra; left=30, right=31
    let stdout = run_bannertext(&[
        "Hello World",
        "--min-total-length",
        "80",
        "--text-alignment",
        "Center",
    ]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 3, "Should have 3 lines");
    let text_line = lines[1];
    assert_eq!(text_line.len(), 80, "Text line should be 80 chars");

    let expected_text_line = format!("**  {:^72}  **", "Hello World");
    assert_eq!(
        text_line, &expected_text_line,
        "Center-aligned text line mismatch"
    );
}

#[test]
fn test_right_alignment_output() {
    // total=20, text_area = 20-4-4 = 12
    // format!("{:>12}", "Hi") = "          Hi" (10 spaces + "Hi")
    let stdout = run_bannertext(&[
        "Hi",
        "--min-total-length",
        "20",
        "--text-alignment",
        "Right",
    ]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 3);
    let text_line = lines[1];
    assert_eq!(text_line.len(), 20, "Text line should be 20 chars");

    let expected_text_line = format!("**  {:>12}  **", "Hi");
    assert_eq!(
        text_line, &expected_text_line,
        "Right-aligned text line mismatch"
    );
}

// ===== Custom Char Exact Output Tests =====

#[test]
fn test_custom_header_footer_chars_output() {
    // "Test" = 4 chars, natural_length = 4+4+4 = 12
    let stdout = run_bannertext(&["Test", "--header-line-char", "=", "--footer-line-char", "-"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "============", "Header should be 12 '=' chars");
    assert_eq!(
        lines[1], "**  Test  **",
        "Text line should still use default '*'"
    );
    assert_eq!(lines[2], "------------", "Footer should be 12 '-' chars");
}

#[test]
fn test_different_text_line_char_output() {
    // "X" = 1 char, natural_length = 4+1+4 = 9
    // text_line_char '-': "--  X  --"
    let stdout = run_bannertext(&["X", "--text-line-char", "-"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "*********", "Header should use '*' (9 chars)");
    assert_eq!(
        lines[1], "--  X  --",
        "Text line should use '-' prefix/suffix"
    );
    assert_eq!(lines[2], "*********", "Footer should use '*' (9 chars)");
}

// ===== Line Count Exact Output Tests =====

#[test]
fn test_multiple_header_lines_output() {
    let stdout = run_bannertext(&["X", "--header-line-count", "3"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(
        lines.len(),
        5,
        "Should have 5 lines (3 header + 1 text + 1 footer)"
    );
    assert!(
        lines[0].chars().all(|c| c == '*'),
        "Line 0 should be header '*'"
    );
    assert!(
        lines[1].chars().all(|c| c == '*'),
        "Line 1 should be header '*'"
    );
    assert!(
        lines[2].chars().all(|c| c == '*'),
        "Line 2 should be header '*'"
    );
    assert!(lines[3].contains('X'), "Line 3 should be text line");
    assert!(
        lines[4].chars().all(|c| c == '*'),
        "Line 4 should be footer '*'"
    );
}

#[test]
fn test_zero_footer_lines_output() {
    let stdout = run_bannertext(&["X", "--footer-line-count", "0"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 2, "Should have 2 lines (1 header + 1 text)");
    assert!(
        lines[0].chars().all(|c| c == '*'),
        "Line 0 should be header"
    );
    assert!(lines[1].contains('X'), "Line 1 should be the text line");
}

// ===== Gap Size Exact Output Tests =====

#[test]
fn test_prefix_gap_zero_output() {
    // "A" = 1 char, prefix_count=2, prefix_gap=0 → prefix="**" (2 chars)
    // suffix: suffix_gap=2, suffix_count=2 → suffix="  **" (4 chars)
    // natural_length = 2+1+4 = 7
    let stdout = run_bannertext(&["A", "--title-prefix-gap-size", "0"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "*******", "Header should be 7 '*' chars");
    assert_eq!(
        lines[1], "**A  **",
        "Text line with zero prefix gap: no space between '**' and 'A'"
    );
    assert_eq!(lines[2], "*******", "Footer should be 7 '*' chars");
}

// ===== Multi-Text Exact Output Tests =====

#[test]
fn test_two_messages_equal_length_exact_output() {
    // "Hello" (5) and "World" (5) → equal length, natural = prefix(4) + 5 + suffix(4) = 13
    let stdout = run_bannertext(&["Hello", "World"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 4, "Two-message banner should have 4 lines");
    assert_eq!(lines[0], "*************", "Header should be 13 '*' chars");
    assert_eq!(lines[1], "**  Hello  **", "First text line mismatch");
    assert_eq!(lines[2], "**  World  **", "Second text line mismatch");
    assert_eq!(lines[3], "*************", "Footer should be 13 '*' chars");
}

#[test]
fn test_two_messages_second_longer_exact_output() {
    // "Hi" (2) and "Hello World" (11) → width driven by second, natural = 19
    let stdout = run_bannertext(&["Hi", "Hello World"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 4, "Two-message banner should have 4 lines");
    assert_eq!(lines[0].len(), 19, "Header should be 19 chars");
    assert_eq!(
        lines[2], "**  Hello World  **",
        "Second text line should be full width"
    );
    assert_eq!(
        lines[1].len(),
        19,
        "First text line should also be 19 chars"
    );
    assert!(
        lines[1].starts_with("**  Hi"),
        "First text line should start with '**  Hi', got: '{}'",
        lines[1]
    );
}

#[test]
fn test_two_messages_first_longer_exact_output() {
    // "Hello World" (11) and "Hi" (2) → width driven by first, natural = 19
    let stdout = run_bannertext(&["Hello World", "Hi"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 4, "Two-message banner should have 4 lines");
    assert_eq!(lines[0].len(), 19, "Header should be 19 chars");
    assert_eq!(
        lines[1], "**  Hello World  **",
        "First text line should be full width"
    );
    assert_eq!(
        lines[2].len(),
        19,
        "Second text line should also be 19 chars"
    );
    assert!(
        lines[2].starts_with("**  Hi"),
        "Second text line should start with '**  Hi', got: '{}'",
        lines[2]
    );
}

#[test]
fn test_three_messages_exact_output() {
    // "A" (1), "BB" (2), "CCC" (3) → width driven by "CCC", natural = prefix(4) + 3 + suffix(4) = 11
    let stdout = run_bannertext(&["A", "BB", "CCC"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(lines.len(), 5, "Three-message banner should have 5 lines");
    assert_eq!(lines[0], "***********", "Header should be 11 '*' chars");
    assert_eq!(lines[4], "***********", "Footer should be 11 '*' chars");
    assert!(
        lines[1].starts_with("**  A"),
        "Line 1 should start with '**  A'"
    );
    assert!(
        lines[2].starts_with("**  BB"),
        "Line 2 should start with '**  BB'"
    );
    assert_eq!(lines[3], "**  CCC  **", "Line 3 should be '**  CCC  **'");
}

#[test]
fn test_single_message_regression_exact_output() {
    // Regression: single positional arg must still produce the same exact 3-line output
    let stdout = run_bannertext(&["Hello World"]);
    let lines: Vec<&str> = stdout.trim_end().lines().collect();

    assert_eq!(
        lines.len(),
        3,
        "Single-message banner should still have 3 lines"
    );
    assert_eq!(lines[0], "*******************", "Header line mismatch");
    assert_eq!(lines[1], "**  Hello World  **", "Text line mismatch");
    assert_eq!(lines[2], "*******************", "Footer line mismatch");
}

// ===== Expected Output File Tests =====

#[test]
fn execute_with_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("-?").env("COLUMNS", "500").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = load_expected_output("Execute_with_help_request_produces_arguments_list");

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_with_full_help_request_produces_arguments_list() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("--help").env("COLUMNS", "500").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = load_expected_output("Execute_with_full_help_request_produces_arguments_list");

    assert_eq!(actual, expected, "Help output does not match expected");
}

#[test]
fn execute_with_text_only_produces_expected_output() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("bob").output().unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = load_expected_output("Execute_with_text_only_produces_expected_output");

    assert_eq!(actual, expected, "Text-only output does not match expected");
}

#[test]
fn execute_with_text_and_min_length_produces_expected_output() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("bob")
        .arg("--min-total-length")
        .arg("80")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected =
        load_expected_output("Execute_with_text_and_min_length_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Text with min-length output does not match expected"
    );
}

#[test]
fn execute_with_multiple_text_lines_produces_expected_output() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("a")
        .arg("bb")
        .arg("ccc")
        .arg("dddd")
        .arg("eeeee")
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected =
        load_expected_output("Execute_with_multiple_text_lines_produces_expected_output");

    assert_eq!(
        actual, expected,
        "Multiple text lines output does not match expected"
    );
}
