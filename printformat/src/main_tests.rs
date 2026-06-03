use super::*;

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

#[test]
fn test_apply_format_no_placeholders() {
    let actual = apply_format("hello", &args(&[])).unwrap();
    assert_eq!(actual, "hello");
}

#[test]
fn test_apply_format_one_placeholder() {
    let actual = apply_format("Hello, {}!", &args(&["World"])).unwrap();
    assert_eq!(actual, "Hello, World!");
}

#[test]
fn test_apply_format_multiple_placeholders() {
    let actual = apply_format("{} + {} = {}", &args(&["1", "2", "3"])).unwrap();
    assert_eq!(actual, "1 + 2 = 3");
}

#[test]
fn test_apply_format_too_few_args() {
    let error = apply_format("Hello, {}!", &args(&[])).unwrap_err();
    assert!(error.contains("placeholder"));
}

#[test]
fn test_apply_format_too_many_args() {
    let error = apply_format("hello", &args(&["extra"])).unwrap_err();
    assert!(error.contains("placeholder"));
}

#[test]
fn test_apply_format_empty_format() {
    let actual = apply_format("", &args(&[])).unwrap();
    assert_eq!(actual, "");
}

#[test]
fn test_apply_format_consecutive_placeholders() {
    let actual = apply_format("{}{}", &args(&["a", "b"])).unwrap();
    assert_eq!(actual, "ab");
}

#[test]
fn test_apply_format_preserves_surrounding_text() {
    let actual = apply_format("pre {} post", &args(&["mid"])).unwrap();
    assert_eq!(actual, "pre mid post");
}
