use super::*;

fn default_banner(texts: &[&str]) -> String {
    generate_banner(
        texts,
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    )
}

// ===== Default Banner Tests =====

#[test]
fn test_default_hello_world() {
    let result = default_banner(&["Hello World"]);
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "*******************");
    assert_eq!(lines[1], "**  Hello World  **");
    assert_eq!(lines[2], "*******************");
}

#[test]
fn test_default_total_length() {
    // natural_length = 2+2+11+2+2 = 19
    let result = default_banner(&["Hello World"]);
    let first_line = result.lines().next().unwrap();
    assert_eq!(first_line.len(), 19);
}

#[test]
fn test_text_line_correct_format() {
    let result = default_banner(&["Hello World"]);
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[1], "**  Hello World  **");
    assert_eq!(lines[1].len(), 19);
}

// ===== Min Length Tests =====

#[test]
fn test_min_total_length_expansion() {
    let result = generate_banner(
        &["Hello World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        80,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[0].len(), 80);
    assert_eq!(lines[2].len(), 80);
    assert_eq!(lines[1].len(), 80);
    // Text left-aligned, padded to fill
    assert!(lines[1].starts_with("**  Hello World"));
    assert!(lines[1].ends_with("**"));
}

#[test]
fn test_min_total_length_no_effect_when_smaller() {
    // min=5 is smaller than natural 19, so no change
    let result = generate_banner(
        &["Hello World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        5,
        0,
    );
    let first_line = result.lines().next().unwrap();
    assert_eq!(first_line.len(), 19);
}

// ===== Alignment Tests =====

#[test]
fn test_center_alignment() {
    let result = generate_banner(
        &["Hello World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Center,
        80,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    let text_line = lines[1];
    assert_eq!(text_line.len(), 80);
    // Strip prefix "**  " and suffix "  **"
    let inner = &text_line[4..76];
    assert_eq!(inner.trim(), "Hello World");
    // Centered: equal (or near-equal) padding on each side
    let left_pad = inner.len() - inner.trim_start().len();
    let right_pad = inner.len() - inner.trim_end().len();
    // For centering, left and right pad should differ by at most 1
    assert!((left_pad as i32 - right_pad as i32).abs() <= 1);
}

#[test]
fn test_right_alignment() {
    let result = generate_banner(
        &["Hello World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Right,
        80,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    let text_line = lines[1];
    assert_eq!(text_line.len(), 80);
    // Should end with "Hello World  **"
    assert!(text_line.ends_with("Hello World  **"));
}

#[test]
fn test_left_alignment_default() {
    let result = generate_banner(
        &["Hello World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        80,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    let text_line = lines[1];
    // Should start with "**  Hello World"
    assert!(text_line.starts_with("**  Hello World"));
}

// ===== Custom Chars Tests =====

#[test]
fn test_custom_header_char() {
    let result = generate_banner(
        &["Test"],
        '#',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert!(lines[0].chars().all(|c| c == '#'));
    assert!(lines[2].chars().all(|c| c == '*'));
}

#[test]
fn test_custom_footer_char() {
    let result = generate_banner(
        &["Test"],
        '*',
        1,
        '-',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert!(lines[0].chars().all(|c| c == '*'));
    assert!(lines[2].chars().all(|c| c == '-'));
}

#[test]
fn test_custom_text_line_char() {
    let result = generate_banner(
        &["Test"],
        '*',
        1,
        '*',
        1,
        '=',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert!(lines[1].starts_with("==  "));
    assert!(lines[1].ends_with("  =="));
}

// ===== Prefix/Suffix Count Tests =====

#[test]
fn test_different_prefix_suffix_counts() {
    let result = generate_banner(
        &["Hi"],
        '*',
        1,
        '*',
        1,
        '*',
        3,
        3,
        1,
        1,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    // natural = 3+1+2+1+3 = 10
    assert_eq!(lines[0].len(), 10);
    assert!(lines[1].starts_with("*** "));
    assert!(lines[1].ends_with(" ***"));
}

#[test]
fn test_zero_prefix_count() {
    let result = generate_banner(
        &["Hi"],
        '*',
        1,
        '*',
        1,
        '*',
        0,
        0,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    // natural = 0+2+2+2+0 = 6
    assert_eq!(lines[0].len(), 6);
    assert_eq!(lines[1], "  Hi  ");
}

// ===== Gap Size Tests =====

#[test]
fn test_different_gap_sizes() {
    let result = generate_banner(
        &["Hi"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        4,
        4,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    // natural = 2+4+2+4+2 = 14
    assert_eq!(lines[0].len(), 14);
    assert!(lines[1].starts_with("**    "));
    assert!(lines[1].ends_with("    **"));
}

#[test]
fn test_zero_gap_sizes() {
    let result = generate_banner(
        &["Hi"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        0,
        0,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    // natural = 2+0+2+0+2 = 6
    assert_eq!(lines[0].len(), 6);
    assert_eq!(lines[1], "**Hi**");
}

// ===== Max Length Truncation Tests =====

#[test]
fn test_max_total_length_truncation() {
    let result = generate_banner(
        &["Hello World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        15,
    );
    let lines: Vec<&str> = result.lines().collect();
    // max=15, prefix_total=4, suffix_total=4, text_area=7
    // text "Hello World" truncated to 7 chars = "Hello W"
    assert_eq!(lines[0].len(), 15);
    assert_eq!(lines[1].len(), 15);
    assert!(lines[1].starts_with("**  Hello W"));
}

#[test]
fn test_max_length_smaller_than_natural() {
    let result = generate_banner(
        &["Hello World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        10,
    );
    let lines: Vec<&str> = result.lines().collect();
    // max=10, prefix_total=4, suffix_total=4, text_area=2
    assert_eq!(lines[0].len(), 10);
    assert_eq!(lines[1].len(), 10);
}

// ===== Empty Text Tests =====

#[test]
fn test_empty_text() {
    let result = generate_banner(
        &[""],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    // natural = 2+2+0+2+2 = 8
    assert_eq!(lines[0].len(), 8);
    assert_eq!(lines[1], "**    **");
}

// ===== Multiple Header/Footer Lines Tests =====

#[test]
fn test_multiple_header_lines() {
    let result = generate_banner(
        &["Hi"],
        '*',
        3,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 5); // 3 header + 1 text + 1 footer
    assert!(lines[0].chars().all(|c| c == '*'));
    assert!(lines[1].chars().all(|c| c == '*'));
    assert!(lines[2].chars().all(|c| c == '*'));
    assert!(lines[3].contains("Hi"));
    assert!(lines[4].chars().all(|c| c == '*'));
}

#[test]
fn test_multiple_footer_lines() {
    let result = generate_banner(
        &["Hi"],
        '*',
        1,
        '*',
        3,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 5); // 1 header + 1 text + 3 footer
    assert!(lines[0].chars().all(|c| c == '*'));
    assert!(lines[1].contains("Hi"));
    assert!(lines[2].chars().all(|c| c == '*'));
    assert!(lines[3].chars().all(|c| c == '*'));
    assert!(lines[4].chars().all(|c| c == '*'));
}

// ===== Zero Header/Footer Lines Tests =====

#[test]
fn test_zero_header_lines() {
    let result = generate_banner(
        &["Hi"],
        '*',
        0,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2); // 0 header + 1 text + 1 footer
    assert!(lines[0].contains("Hi"));
    assert!(lines[1].chars().all(|c| c == '*'));
}

#[test]
fn test_zero_footer_lines() {
    let result = generate_banner(
        &["Hi"],
        '*',
        1,
        '*',
        0,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2); // 1 header + 1 text + 0 footer
    assert!(lines[0].chars().all(|c| c == '*'));
    assert!(lines[1].contains("Hi"));
}

#[test]
fn test_zero_header_and_footer_lines() {
    let result = generate_banner(
        &["Hi"],
        '*',
        0,
        '*',
        0,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains("Hi"));
}

// ===== Multi-Text Tests =====

#[test]
fn test_multi_text_two_equal_length() {
    // "Hello" and "World" are both 5 chars → natural = prefix(4) + 5 + suffix(4) = 13
    let result = generate_banner(
        &["Hello", "World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 4); // 1 header + 2 text + 1 footer
    assert_eq!(lines[0].len(), 13);
    assert_eq!(lines[1], "**  Hello  **");
    assert_eq!(lines[2], "**  World  **");
    assert_eq!(lines[3], "*************");
}

#[test]
fn test_multi_text_second_longer() {
    // "Hi" (2) and "Hello" (5) → width driven by "Hello", natural = 13
    let result = generate_banner(
        &["Hi", "Hello"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0].len(), 13);
    assert_eq!(lines[1].len(), 13);
    assert!(
        lines[1].starts_with("**  Hi"),
        "First line should start with '**  Hi'"
    );
    assert_eq!(lines[2], "**  Hello  **");
    assert_eq!(lines[3], "*************");
}

#[test]
fn test_multi_text_first_longer() {
    // "Hello" (5) and "Hi" (2) → width driven by "Hello", natural = 13
    let result = generate_banner(
        &["Hello", "Hi"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0].len(), 13);
    assert_eq!(lines[1], "**  Hello  **");
    assert_eq!(lines[2].len(), 13);
    assert!(
        lines[2].starts_with("**  Hi"),
        "Second line should start with '**  Hi'"
    );
    assert_eq!(lines[3], "*************");
}

#[test]
fn test_multi_text_three_texts() {
    // "A" (1), "BB" (2), "CCC" (3) → width driven by "CCC", natural = 4+3+4 = 11
    let result = generate_banner(
        &["A", "BB", "CCC"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 5); // 1 header + 3 text + 1 footer
    assert_eq!(lines[0].len(), 11);
    assert_eq!(lines[4].len(), 11);
    assert!(
        lines[1].starts_with("**  A"),
        "First text line should start with '**  A'"
    );
    assert!(
        lines[2].starts_with("**  BB"),
        "Second text line should start with '**  BB'"
    );
    assert_eq!(lines[3], "**  CCC  **");
}

#[test]
fn test_multi_text_with_empty_entry() {
    // "Hello" (5) and "" (0) → width driven by "Hello", natural = 13
    let result = generate_banner(
        &["Hello", ""],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0].len(), 13);
    assert_eq!(lines[1], "**  Hello  **");
    // Empty text left-aligned in width 5 → 5 spaces
    assert_eq!(lines[2].len(), 13);
    assert_eq!(lines[2], "**         **"); // "**" + "  " + "     " + "  " + "**"
}

#[test]
fn test_single_text_slice_matches_original_behaviour() {
    // Regression: single-element slice should produce same output as before
    let result = generate_banner(
        &["Hello World"],
        '*',
        1,
        '*',
        1,
        '*',
        2,
        2,
        2,
        2,
        TextAlignment::Left,
        0,
        0,
    );
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "*******************");
    assert_eq!(lines[1], "**  Hello World  **");
    assert_eq!(lines[2], "*******************");
}
