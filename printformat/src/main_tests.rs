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

#[test]
fn test_translate_csharp_format_basic_placeholders() {
    let actual = translate_csharp_format("{0} is {1} years old").unwrap();
    assert_eq!(actual, "{0} is {1} years old");
}

#[test]
fn test_translate_csharp_format_alignment() {
    let actual = translate_csharp_format("{0,-10} | {1,10}").unwrap();
    assert_eq!(actual, "{0:<10} | {1:>10}");
}

#[test]
fn test_translate_csharp_format_numeric_specifiers() {
    let actual = translate_csharp_format("{0:D5} {1:F2} {2:E3}").unwrap();
    assert_eq!(actual, "{0:0>5} {1:.2} {2:.3E}");
}

#[test]
fn test_translate_csharp_format_preserves_escaped_braces() {
    let actual = translate_csharp_format("{{value}} {0}").unwrap();
    assert_eq!(actual, "{{value}} {0}");
}

#[test]
fn test_translate_csharp_format_rejects_hex() {
    let error = translate_csharp_format("{0:X}").unwrap_err();
    assert_eq!(error, "C# format specifier 'X' (hex) is not supported");
}

mod csharp_tests {
    use super::translate_csharp_format;
    use super::{apply_format, args};

    #[test]
    fn test_translate_csharp_passthrough_single() {
        let result = translate_csharp_format("{0}").unwrap();
        assert_eq!(result, "{0}");
    }

    #[test]
    fn test_translate_csharp_passthrough_multiple() {
        let result = translate_csharp_format("{0} {1} {2}").unwrap();
        assert_eq!(result, "{0} {1} {2}");
    }

    #[test]
    fn test_translate_csharp_left_align() {
        let result = translate_csharp_format("{0,-10}").unwrap();
        assert_eq!(result, "{0:<10}");
    }

    #[test]
    fn test_translate_csharp_right_align() {
        let result = translate_csharp_format("{0,10}").unwrap();
        assert_eq!(result, "{0:>10}");
    }

    #[test]
    fn test_translate_csharp_decimal_format_uppercase() {
        let result = translate_csharp_format("{0:D3}").unwrap();
        assert_eq!(result, "{0:0>3}");
    }

    #[test]
    fn test_translate_csharp_decimal_format_lowercase() {
        let result = translate_csharp_format("{0:d5}").unwrap();
        assert_eq!(result, "{0:0>5}");
    }

    #[test]
    fn test_translate_csharp_fixed_point_uppercase() {
        let result = translate_csharp_format("{0:F2}").unwrap();
        assert_eq!(result, "{0:.2}");
    }

    #[test]
    fn test_translate_csharp_fixed_point_lowercase() {
        let result = translate_csharp_format("{0:f4}").unwrap();
        assert_eq!(result, "{0:.4}");
    }

    #[test]
    fn test_translate_csharp_general_format_passthrough() {
        let result = translate_csharp_format("{0:G}").unwrap();
        assert_eq!(result, "{0:G}");
    }

    #[test]
    fn test_translate_csharp_hex_format_returns_error() {
        let error = translate_csharp_format("{0:X}").unwrap_err();
        assert!(error.contains("hex") || error.contains("X"));
    }

    #[test]
    fn test_translate_csharp_number_format_returns_error() {
        let error = translate_csharp_format("{0:N}").unwrap_err();
        assert!(error.contains("thousands") || error.contains("N"));
    }

    #[test]
    fn test_translate_csharp_currency_format_returns_error() {
        let error = translate_csharp_format("{0:C}").unwrap_err();
        assert!(error.contains("currency") || error.contains("C"));
    }

    #[test]
    fn test_translate_csharp_preserves_escaped_braces() {
        let result = translate_csharp_format("{{0}}").unwrap();
        assert_eq!(result, "{{0}}");
    }

    #[test]
    fn test_translate_csharp_mixed_alignment() {
        let result = translate_csharp_format("{0,-10} | {1,10}").unwrap();
        assert_eq!(result, "{0:<10} | {1:>10}");
    }

    #[test]
    fn test_apply_format_with_csharp_positional_placeholder() {
        let actual = apply_format("{0}", &args(&["hello"])).unwrap();
        assert_eq!(actual, "hello");
    }

    #[test]
    fn test_apply_format_with_translated_csharp_left_align() {
        let translated = translate_csharp_format("{0,-10}").unwrap();
        let actual = apply_format(&translated, &args(&["hello"])).unwrap();
        assert_eq!(actual, "hello     ");
    }

    #[test]
    fn test_apply_format_with_translated_csharp_right_align() {
        let translated = translate_csharp_format("{0,10}").unwrap();
        let actual = apply_format(&translated, &args(&["hello"])).unwrap();
        assert_eq!(actual, "     hello");
    }
}
