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

#[test]
fn test_apply_format_right_align() {
    let actual = apply_format("{:>10}", &args(&["hello"])).unwrap();
    assert_eq!(actual, "     hello");
}

#[test]
fn test_apply_format_left_align() {
    let actual = apply_format("{:<10}", &args(&["hello"])).unwrap();
    assert_eq!(actual, "hello     ");
}

#[test]
fn test_apply_format_center_align() {
    let actual = apply_format("{:^11}", &args(&["hello"])).unwrap();
    assert_eq!(actual, "   hello   ");
}

#[test]
fn test_apply_format_fill_char() {
    let actual = apply_format("{:*^11}", &args(&["hello"])).unwrap();
    assert_eq!(actual, "***hello***");
}

#[test]
fn test_apply_format_zero_pad() {
    let actual = apply_format("{:0>5}", &args(&["42"])).unwrap();
    assert_eq!(actual, "00042");
}

#[test]
fn test_apply_format_width_no_align() {
    let actual = apply_format("{:10}", &args(&["hi"])).unwrap();
    assert_eq!(actual.len(), 10);
    assert!(actual.contains("hi"));
}

#[test]
fn test_apply_format_escaped_braces() {
    let actual = apply_format("{{}} and {}", &args(&["value"])).unwrap();
    assert_eq!(actual, "{} and value");
}

#[test]
fn test_apply_format_multiple_with_alignment() {
    let actual = apply_format("{:>5} | {:<5}", &args(&["a", "b"])).unwrap();
    assert_eq!(actual, "    a | b    ");
}

// ===== Property Tests =====

mod properties {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn escaped_text_formats_back_to_itself(text in r"\PC{0,40}") {
            let escaped = text.replace('{', "{{").replace('}', "}}");
            prop_assert_eq!(apply_format(&escaped, &[]), Ok(text));
        }

        #[test]
        fn automatic_placeholders_take_the_arguments_in_order(
            values in proptest::collection::vec(r"\PC{0,10}", 0..6),
        ) {
            let format_str = "{}".repeat(values.len());
            prop_assert_eq!(apply_format(&format_str, &values), Ok(values.concat()));
        }

        #[test]
        fn the_wrong_number_of_arguments_is_an_error(
            values in proptest::collection::vec("[a-z]{0,5}", 0..6),
            difference in 1..4usize,
            fewer: bool,
        ) {
            let placeholders = match values.len().checked_sub(difference) {
                Some(count) if fewer => count,
                _ => values.len() + difference,
            };
            let format_str = "{}".repeat(placeholders);
            prop_assert!(apply_format(&format_str, &values).is_err());
        }
    }
}
