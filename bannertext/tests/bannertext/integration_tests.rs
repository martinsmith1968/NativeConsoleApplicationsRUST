use assert_cmd::Command;
use predicates::prelude::*;

// ===== CLI Help and Version Tests =====

#[test]
fn test_cli_help_flag() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Display a Text Banner"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version_flag() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("bannertext"));
}

#[test]
fn test_cli_help_short_alias() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("-?")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version_short_alias() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("-!")
        .assert()
        .success()
        .stdout(predicate::str::contains("bannertext"));
}

// ===== Default Output Tests =====

#[test]
fn test_cli_default_hello_world_three_lines() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("Hello World").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(lines.len(), 3, "Default banner should have 3 lines");
}

#[test]
fn test_cli_default_hello_world_header_line() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World")
        .assert()
        .success()
        .stdout(predicate::str::contains("*******************"));
}

#[test]
fn test_cli_default_hello_world_text_line() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World")
        .assert()
        .success()
        .stdout(predicate::str::contains("**  Hello World  **"));
}

#[test]
fn test_cli_no_args_fails() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.assert().failure();
}

// ===== Header/Footer Line Character Tests =====

#[test]
fn test_cli_header_line_char_hash() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--header-line-char")
        .arg("#")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert!(
        lines[0].chars().all(|c| c == '#'),
        "Header line should contain only '#', got: '{}'",
        lines[0]
    );
    assert!(
        lines[1].starts_with("**"),
        "Text line should still use default '*'"
    );
}

#[test]
fn test_cli_footer_line_char_hash() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--footer-line-char")
        .arg("#")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert!(
        lines[2].chars().all(|c| c == '#'),
        "Footer line should contain only '#', got: '{}'",
        lines[2]
    );
    assert!(
        lines[0].chars().all(|c| c == '*'),
        "Header line should still use default '*'"
    );
}

#[test]
fn test_cli_text_line_char_dash() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World")
        .arg("--text-line-char")
        .arg("-")
        .assert()
        .success()
        .stdout(predicate::str::contains("--  Hello World  --"));
}

// ===== Header/Footer Line Count Tests =====

#[test]
fn test_cli_header_line_count_zero() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--header-line-count")
        .arg("0")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(
        lines.len(),
        2,
        "With header-line-count 0, should have 2 lines (text + footer)"
    );
    assert!(
        lines[0].contains("Hello World"),
        "First line should be the text line"
    );
}

#[test]
fn test_cli_header_line_count_three() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--header-line-count")
        .arg("3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(
        lines.len(),
        5,
        "With header-line-count 3, should have 5 lines (3 header + text + footer)"
    );
    assert!(
        lines[0].chars().all(|c| c == '*'),
        "Line 0 should be a header"
    );
    assert!(
        lines[1].chars().all(|c| c == '*'),
        "Line 1 should be a header"
    );
    assert!(
        lines[2].chars().all(|c| c == '*'),
        "Line 2 should be a header"
    );
    assert!(
        lines[3].contains("Hello World"),
        "Line 3 should be text line"
    );
    assert!(
        lines[4].chars().all(|c| c == '*'),
        "Line 4 should be footer"
    );
}

#[test]
fn test_cli_footer_line_count_zero() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--footer-line-count")
        .arg("0")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(
        lines.len(),
        2,
        "With footer-line-count 0, should have 2 lines (header + text)"
    );
    assert!(
        lines[1].contains("Hello World"),
        "Last line should be the text line"
    );
}

#[test]
fn test_cli_footer_line_count_two() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--footer-line-count")
        .arg("2")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(
        lines.len(),
        4,
        "With footer-line-count 2, should have 4 lines (header + text + 2 footer)"
    );
}

// ===== Min/Max Total Length Tests =====

#[test]
fn test_cli_min_total_length_40() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--min-total-length")
        .arg("40")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.trim_end().lines() {
        assert!(
            line.len() >= 40,
            "All lines should be >= 40 chars, got '{}' (len {})",
            line,
            line.len()
        );
    }
}

#[test]
fn test_cli_min_total_length_80_exact() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--min-total-length")
        .arg("80")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.trim_end().lines() {
        assert_eq!(
            line.len(),
            80,
            "All lines should be exactly 80 chars, got len {} for '{}'",
            line.len(),
            line
        );
    }
}

#[test]
fn test_cli_max_total_length_15() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--max-total-length")
        .arg("15")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.trim_end().lines() {
        assert!(
            line.len() <= 15,
            "All lines should be <= 15 chars, got '{}' (len {})",
            line,
            line.len()
        );
    }
}

#[test]
fn test_cli_min_wins_over_natural_length() {
    // natural_length = 19, min = 40: min expands the banner
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--min-total-length")
        .arg("40")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let first_line = stdout.trim_end().lines().next().unwrap();
    assert_eq!(
        first_line.len(),
        40,
        "Banner should expand to min-total-length"
    );
}

#[test]
fn test_cli_max_truncates_when_natural_exceeds() {
    // natural_length = 19, max = 15: max truncates the banner
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--max-total-length")
        .arg("15")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let first_line = stdout.trim_end().lines().next().unwrap();
    assert_eq!(
        first_line.len(),
        15,
        "Banner should truncate to max-total-length"
    );
}

// ===== Text Alignment Tests =====

#[test]
fn test_cli_text_alignment_left_default() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World")
        .arg("--min-total-length")
        .arg("40")
        .assert()
        .success()
        .stdout(predicate::str::contains("**  Hello World"));
}

#[test]
fn test_cli_text_alignment_right() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--min-total-length")
        .arg("40")
        .arg("--text-alignment")
        .arg("Right")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert!(
        lines[1].ends_with("Hello World  **"),
        "Right-aligned text should end with 'Hello World  **', got: '{}'",
        lines[1]
    );
}

#[test]
fn test_cli_text_alignment_center() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--min-total-length")
        .arg("40")
        .arg("--text-alignment")
        .arg("Center")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    let text_line = lines[1];
    // Strip prefix "**  " (4 bytes) and suffix "  **" (4 bytes)
    let inner = &text_line[4..text_line.len() - 4];
    assert_eq!(
        inner.trim(),
        "Hello World",
        "Inner text should be 'Hello World'"
    );
    let left_pad = inner.len() - inner.trim_start().len();
    let right_pad = inner.len() - inner.trim_end().len();
    assert!(
        (left_pad as i32 - right_pad as i32).abs() <= 1,
        "Text should be centered (left_pad={}, right_pad={})",
        left_pad,
        right_pad
    );
}

#[test]
fn test_cli_text_alignment_center_with_min_40() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--min-total-length")
        .arg("40")
        .arg("--text-alignment")
        .arg("Center")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.trim_end().lines() {
        assert_eq!(line.len(), 40, "All lines should be 40 chars wide");
    }
}

// ===== Prefix/Suffix Count Tests =====

#[test]
fn test_cli_title_prefix_count_one() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World")
        .arg("--title-prefix-count")
        .arg("1")
        .assert()
        .success()
        .stdout(predicate::str::contains("*  Hello World  **"));
}

#[test]
fn test_cli_title_prefix_count_four() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World")
        .arg("--title-prefix-count")
        .arg("4")
        .assert()
        .success()
        .stdout(predicate::str::contains("****  Hello World  **"));
}

#[test]
fn test_cli_title_suffix_count_zero() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("--title-suffix-count")
        .arg("0")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert!(
        lines[1].starts_with("**  Hello World"),
        "Text line should start with prefix + text"
    );
    assert!(
        !lines[1].ends_with("**"),
        "Text line should not end with '**' when suffix-count is 0"
    );
}

// ===== Gap Size Tests =====

#[test]
fn test_cli_title_prefix_gap_size_zero() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World")
        .arg("--title-prefix-gap-size")
        .arg("0")
        .assert()
        .success()
        .stdout(predicate::str::contains("**Hello World"));
}

#[test]
fn test_cli_title_suffix_gap_size_four() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World")
        .arg("--title-suffix-gap-size")
        .arg("4")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World    **"));
}

// ===== Short Option Tests =====

#[test]
fn test_cli_short_header_char() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("Hello World").arg("-H").arg("#").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert!(
        lines[0].chars().all(|c| c == '#'),
        "Header line should use '#' with -H short option, got: '{}'",
        lines[0]
    );
}

#[test]
fn test_cli_short_alignment_center_with_min_length() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd
        .arg("Hello World")
        .arg("-a")
        .arg("Center")
        .arg("-m")
        .arg("40")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(lines[0].len(), 40, "Line length should be 40 with -m 40");
    let text_line = lines[1];
    let inner = &text_line[4..text_line.len() - 4];
    let left_pad = inner.len() - inner.trim_start().len();
    let right_pad = inner.len() - inner.trim_end().len();
    assert!(
        (left_pad as i32 - right_pad as i32).abs() <= 1,
        "Text should be centered (left_pad={}, right_pad={})",
        left_pad,
        right_pad
    );
}

// ===== Exit Code Tests =====

#[test]
fn test_cli_success_exit_code() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello World").assert().code(0);
}

#[test]
fn test_cli_help_exit_code() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("--help").assert().code(0);
}

#[test]
fn test_cli_version_exit_code() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("--version").assert().code(0);
}

#[test]
fn test_cli_no_args_exit_code() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.assert().failure();
}

// ===== Multi-Text (Multiple Positional Args) Tests =====

#[test]
fn test_cli_two_args_exits_zero() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello").arg("World").assert().success();
}

#[test]
fn test_cli_two_args_produces_four_lines() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("Hello").arg("World").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(
        lines.len(),
        4,
        "Two-arg banner should have 4 lines (header + 2 text + footer)"
    );
}

#[test]
fn test_cli_two_args_both_text_lines_present() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Hello")
        .arg("World")
        .assert()
        .success()
        .stdout(predicate::str::contains("**  Hello  **"))
        .stdout(predicate::str::contains("**  World  **"));
}

#[test]
fn test_cli_three_args_exits_zero() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    cmd.arg("Line1")
        .arg("Line2")
        .arg("Line3")
        .assert()
        .success();
}

#[test]
fn test_cli_three_args_produces_five_lines() {
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("Line1").arg("Line2").arg("Line3").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(
        lines.len(),
        5,
        "Three-arg banner should have 5 lines (header + 3 text + footer)"
    );
}

#[test]
fn test_cli_width_driven_by_longest_text() {
    // "Hi" (2 chars) and "Hello World" (11 chars) → width = prefix(4) + 11 + suffix(4) = 19
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("Hi").arg("Hello World").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(lines.len(), 4);
    for line in &lines {
        assert_eq!(
            line.len(),
            19,
            "All lines should be 19 chars (driven by 'Hello World'), got '{}' (len {})",
            line,
            line.len()
        );
    }
}

#[test]
fn test_cli_single_arg_still_produces_three_lines() {
    // Regression: single positional arg should still produce 3 lines
    let mut cmd = Command::cargo_bin("bannertext").unwrap();
    let output = cmd.arg("Hello World").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim_end().lines().collect();
    assert_eq!(
        lines.len(),
        3,
        "Single-arg banner should still produce 3 lines"
    );
}
