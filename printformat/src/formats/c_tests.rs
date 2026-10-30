use super::*;

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}

#[test]
fn test_extract_specifiers_string() {
    assert_eq!(extract_c_specifiers("%s").unwrap(), vec!['s']);
}

#[test]
fn test_extract_specifiers_integer() {
    assert_eq!(extract_c_specifiers("%d").unwrap(), vec!['d']);
}

#[test]
fn test_extract_specifiers_multiple() {
    assert_eq!(
        extract_c_specifiers("%s is %d years old").unwrap(),
        vec!['s', 'd']
    );
}

#[test]
fn test_extract_specifiers_escaped_percent() {
    assert_eq!(extract_c_specifiers("100%%").unwrap(), Vec::<char>::new());
}

#[test]
fn test_extract_specifiers_with_flags_and_width() {
    assert_eq!(extract_c_specifiers("%10s").unwrap(), vec!['s']);
    assert_eq!(extract_c_specifiers("%-10s").unwrap(), vec!['s']);
    assert_eq!(extract_c_specifiers("%05d").unwrap(), vec!['d']);
}

#[test]
fn test_extract_specifiers_with_precision() {
    assert_eq!(extract_c_specifiers("%.2f").unwrap(), vec!['f']);
}

#[test]
fn test_extract_specifiers_unsupported_n() {
    assert!(extract_c_specifiers("%n").is_err());
}

#[test]
fn test_extract_specifiers_unsupported_p() {
    assert!(extract_c_specifiers("%p").is_err());
}

#[test]
fn test_apply_c_format_string() {
    assert_eq!(apply_c_format("%s", &args(&["hello"])).unwrap(), "hello");
}

#[test]
fn test_apply_c_format_integer() {
    assert_eq!(apply_c_format("%d", &args(&["42"])).unwrap(), "42");
}

#[test]
fn test_apply_c_format_zero_padded_integer() {
    assert_eq!(apply_c_format("%05d", &args(&["42"])).unwrap(), "00042");
}

#[test]
fn test_apply_c_format_float() {
    assert_eq!(apply_c_format("%.2f", &args(&["3.14159"])).unwrap(), "3.14");
}

#[test]
fn test_apply_c_format_right_aligned_string() {
    assert_eq!(
        apply_c_format("%10s", &args(&["hi"])).unwrap(),
        "        hi"
    );
}

#[test]
fn test_apply_c_format_left_aligned_string() {
    assert_eq!(
        apply_c_format("%-10s", &args(&["hi"])).unwrap(),
        "hi        "
    );
}

#[test]
fn test_apply_c_format_hex_lowercase() {
    assert_eq!(apply_c_format("%x", &args(&["255"])).unwrap(), "ff");
}

#[test]
fn test_apply_c_format_hex_uppercase() {
    assert_eq!(apply_c_format("%X", &args(&["255"])).unwrap(), "FF");
}

#[test]
fn test_apply_c_format_octal() {
    assert_eq!(apply_c_format("%o", &args(&["8"])).unwrap(), "10");
}

#[test]
fn test_apply_c_format_literal_percent() {
    assert_eq!(apply_c_format("100%%", &args(&[])).unwrap(), "100%");
}

#[test]
fn test_apply_c_format_char() {
    assert_eq!(apply_c_format("%c", &args(&["A"])).unwrap(), "A");
}

#[test]
fn test_apply_c_format_multiple_mixed() {
    assert_eq!(
        apply_c_format("%s is %d years old", &args(&["Alice", "30"])).unwrap(),
        "Alice is 30 years old"
    );
}

#[test]
fn test_apply_c_format_too_few_args() {
    let err = apply_c_format("%d %d", &args(&["1"])).unwrap_err();
    assert!(err.contains("placeholder"));
}

#[test]
fn test_apply_c_format_too_many_args() {
    let err = apply_c_format("%d", &args(&["1", "2"])).unwrap_err();
    assert!(err.contains("placeholder"));
}

#[test]
fn test_apply_c_format_invalid_integer() {
    let err = apply_c_format("%d", &args(&["notanumber"])).unwrap_err();
    assert!(err.contains("integer"));
}

#[test]
fn test_apply_c_format_invalid_float() {
    let err = apply_c_format("%f", &args(&["notanumber"])).unwrap_err();
    assert!(err.contains("number"));
}
