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

    // Numeric zero-padding (D specifier — equivalent to C# "00000" custom format)

    #[test]
    fn test_csharp_d5_zero_pads_short_number() {
        let translated = translate_csharp_format("{0:D5}").unwrap();
        let actual = apply_format(&translated, &args(&["42"])).unwrap();
        assert_eq!(actual, "00042");
    }

    #[test]
    fn test_csharp_d5_single_digit() {
        let translated = translate_csharp_format("{0:D5}").unwrap();
        let actual = apply_format(&translated, &args(&["7"])).unwrap();
        assert_eq!(actual, "00007");
    }

    #[test]
    fn test_csharp_d5_exact_width() {
        let translated = translate_csharp_format("{0:D5}").unwrap();
        let actual = apply_format(&translated, &args(&["12345"])).unwrap();
        assert_eq!(actual, "12345");
    }

    #[test]
    fn test_csharp_d5_longer_than_width_no_truncation() {
        let translated = translate_csharp_format("{0:D5}").unwrap();
        let actual = apply_format(&translated, &args(&["123456"])).unwrap();
        assert_eq!(actual, "123456");
    }

    #[test]
    fn test_csharp_d3_zero_pads_single_digit() {
        let translated = translate_csharp_format("{0:D3}").unwrap();
        let actual = apply_format(&translated, &args(&["7"])).unwrap();
        assert_eq!(actual, "007");
    }

    #[test]
    fn test_csharp_d1_no_pad_needed() {
        let translated = translate_csharp_format("{0:D1}").unwrap();
        let actual = apply_format(&translated, &args(&["42"])).unwrap();
        assert_eq!(actual, "42");
    }

    #[test]
    fn test_csharp_lowercase_d5() {
        let translated = translate_csharp_format("{0:d5}").unwrap();
        let actual = apply_format(&translated, &args(&["42"])).unwrap();
        assert_eq!(actual, "00042");
    }

    #[test]
    fn test_csharp_d_no_precision_no_padding() {
        let translated = translate_csharp_format("{0:D}").unwrap();
        let actual = apply_format(&translated, &args(&["42"])).unwrap();
        assert_eq!(actual, "42");
    }

    #[test]
    fn test_csharp_d5_multiple_args() {
        let translated = translate_csharp_format("{0:D5} and {1:D3}").unwrap();
        let actual = apply_format(&translated, &args(&["42", "7"])).unwrap();
        assert_eq!(actual, "00042 and 007");
    }

    // Text alignment / padding

    #[test]
    fn test_csharp_left_align_short_text() {
        // "hi" left-aligned in width 10 → "hi" + 8 spaces
        let translated = translate_csharp_format("{0,-10}").unwrap();
        let actual = apply_format(&translated, &args(&["hi"])).unwrap();
        assert_eq!(actual, "hi        ");
    }

    #[test]
    fn test_csharp_right_align_short_text() {
        // "hi" right-aligned in width 10 → 8 spaces + "hi"
        let translated = translate_csharp_format("{0,10}").unwrap();
        let actual = apply_format(&translated, &args(&["hi"])).unwrap();
        assert_eq!(actual, "        hi");
    }

    #[test]
    fn test_csharp_left_align_text_longer_than_width_no_truncation() {
        // "toolong" is 7 chars, width 5 — no truncation, full string returned
        let translated = translate_csharp_format("{0,-5}").unwrap();
        let actual = apply_format(&translated, &args(&["toolong"])).unwrap();
        assert_eq!(actual, "toolong");
    }

    #[test]
    fn test_csharp_right_align_text_longer_than_width_no_truncation() {
        let translated = translate_csharp_format("{0,5}").unwrap();
        let actual = apply_format(&translated, &args(&["toolong"])).unwrap();
        assert_eq!(actual, "toolong");
    }

    #[test]
    fn test_csharp_align_width_one_exact_fit() {
        let translated = translate_csharp_format("{0,1}").unwrap();
        let actual = apply_format(&translated, &args(&["x"])).unwrap();
        assert_eq!(actual, "x");
    }

    #[test]
    fn test_csharp_mixed_alignment_table_row() {
        // Simulates a table row: left-padded label, right-padded value
        let translated = translate_csharp_format("{0,-10} | {1,10}").unwrap();
        let actual = apply_format(&translated, &args(&["Name", "Value"])).unwrap();
        assert_eq!(actual, "Name       |      Value");
    }

    #[test]
    fn test_csharp_multiple_left_aligned_columns() {
        // Three columns each 8 chars wide, left-aligned
        let translated = translate_csharp_format("{0,-8} {1,-8} {2,-8}").unwrap();
        let actual = apply_format(&translated, &args(&["abc", "de", "f"])).unwrap();
        assert_eq!(actual, "abc      de       f       ");
    }

    #[test]
    fn test_csharp_right_align_zero_value() {
        let translated = translate_csharp_format("{0,5}").unwrap();
        let actual = apply_format(&translated, &args(&["0"])).unwrap();
        assert_eq!(actual, "    0");
    }

    #[test]
    fn test_csharp_left_align_empty_string() {
        // Empty string left-aligned in width 4 → 4 spaces
        let translated = translate_csharp_format("{0,-4}").unwrap();
        let actual = apply_format(&translated, &args(&[""])).unwrap();
        assert_eq!(actual, "    ");
    }
}

mod c_format_tests {
    use super::{apply_c_format, extract_c_specifiers};

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
        assert_eq!(apply_c_format("%10s", &args(&["hi"])).unwrap(), "        hi");
    }

    #[test]
    fn test_apply_c_format_left_aligned_string() {
        assert_eq!(apply_c_format("%-10s", &args(&["hi"])).unwrap(), "hi        ");
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
}
