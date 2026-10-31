use super::*;
use crate::formats::culture::Culture;

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn format(format_str: &str, values: &[&str]) -> Result<String, String> {
    format_in(Culture::EnGb, format_str, values)
}

fn format_in(culture: Culture, format_str: &str, values: &[&str]) -> Result<String, String> {
    apply_csharp_format(format_str, &args(values), &culture.number_format())
}

// ===== Placeholders, escapes and argument counts =====

#[test]
fn test_basic_placeholders() {
    assert_eq!(
        format("{0} is {1} years old", &["Alice", "30"]).unwrap(),
        "Alice is 30 years old"
    );
}

#[test]
fn test_repeated_and_reordered_placeholders() {
    assert_eq!(format("{1} {0} {1}", &["a", "b"]).unwrap(), "b a b");
}

#[test]
fn test_escaped_braces() {
    assert_eq!(format("{{value}} {0}", &["x"]).unwrap(), "{value} x");
    assert_eq!(format("{{0}}", &[]).unwrap(), "{0}");
}

#[test]
fn test_too_few_arguments() {
    let error = format("{0} {1}", &["a"]).unwrap_err();
    assert!(error.contains("placeholder"));
}

#[test]
fn test_too_many_arguments() {
    let error = format("{0}", &["a", "b"]).unwrap_err();
    assert!(error.contains("placeholder"));
}

#[test]
fn test_invalid_placeholder() {
    assert!(format("{x}", &["a"]).is_err());
    assert!(format("{0;}", &["a"]).is_err());
}

#[test]
fn test_unsupported_specifier() {
    let error = format("{0:Q}", &["1"]).unwrap_err();
    assert_eq!(error, "C# format specifier 'Q' is not supported");
}

#[test]
fn test_precision_too_large() {
    let error = format("{0:F100}", &["1"]).unwrap_err();
    assert!(error.contains("too large"));
}

// ===== Alignment =====

#[test]
fn test_left_align() {
    assert_eq!(format("{0,-10}", &["hello"]).unwrap(), "hello     ");
}

#[test]
fn test_right_align() {
    assert_eq!(format("{0,10}", &["hello"]).unwrap(), "     hello");
}

#[test]
fn test_align_text_longer_than_width_no_truncation() {
    assert_eq!(format("{0,-5}", &["toolong"]).unwrap(), "toolong");
    assert_eq!(format("{0,5}", &["toolong"]).unwrap(), "toolong");
}

#[test]
fn test_align_width_one_exact_fit() {
    assert_eq!(format("{0,1}", &["x"]).unwrap(), "x");
}

#[test]
fn test_mixed_alignment_table_row() {
    assert_eq!(
        format("{0,-10} | {1,10}", &["Name", "Value"]).unwrap(),
        "Name       |      Value"
    );
}

#[test]
fn test_multiple_left_aligned_columns() {
    assert_eq!(
        format("{0,-8} {1,-8} {2,-8}", &["abc", "de", "f"]).unwrap(),
        "abc      de       f       "
    );
}

#[test]
fn test_left_align_empty_string() {
    assert_eq!(format("{0,-4}", &[""]).unwrap(), "    ");
}

#[test]
fn test_alignment_with_zero_padding() {
    assert_eq!(format("{0,8:D5}", &["42"]).unwrap(), "   00042");
}

// ===== D (decimal) =====

#[test]
fn test_d_zero_pads() {
    assert_eq!(format("{0:D5}", &["42"]).unwrap(), "00042");
    assert_eq!(format("{0:D5}", &["7"]).unwrap(), "00007");
    assert_eq!(format("{0:D3}", &["7"]).unwrap(), "007");
    assert_eq!(format("{0:d5}", &["42"]).unwrap(), "00042");
}

#[test]
fn test_d_no_truncation_or_padding() {
    assert_eq!(format("{0:D5}", &["12345"]).unwrap(), "12345");
    assert_eq!(format("{0:D5}", &["123456"]).unwrap(), "123456");
    assert_eq!(format("{0:D1}", &["42"]).unwrap(), "42");
    assert_eq!(format("{0:D}", &["42"]).unwrap(), "42");
}

#[test]
fn test_d_negative_keeps_sign_before_padding() {
    assert_eq!(format("{0:D5}", &["-42"]).unwrap(), "-00042");
}

#[test]
fn test_d_multiple_args() {
    assert_eq!(
        format("{0:D5} and {1:D3}", &["42", "7"]).unwrap(),
        "00042 and 007"
    );
}

#[test]
fn test_d_rejects_non_integer() {
    assert!(format("{0:D5}", &["abc"]).unwrap_err().contains("integer"));
    assert!(format("{0:D5}", &["3.5"]).unwrap_err().contains("integer"));
}

// ===== F (fixed-point) =====

#[test]
fn test_f_rounds_to_precision() {
    assert_eq!(format("{0:F2}", &["3.14159"]).unwrap(), "3.14");
    assert_eq!(format("{0:f4}", &["3.14159"]).unwrap(), "3.1416");
    assert_eq!(format("{0:F0}", &["3.7"]).unwrap(), "4");
}

#[test]
fn test_f_rounds_halves_away_from_zero() {
    assert_eq!(format("{0:F0}", &["2.5"]).unwrap(), "3");
    assert_eq!(format("{0:F0}", &["-2.5"]).unwrap(), "-3");
    assert_eq!(format("{0:F2}", &["0.125"]).unwrap(), "0.13");
    assert_eq!(format("{0:F1}", &["9.95"]).unwrap(), "9.9");
    assert_eq!(format("{0:F2}", &["9.999"]).unwrap(), "10.00");
}

#[test]
fn test_f_defaults_to_two_decimals() {
    assert_eq!(format("{0:F}", &["2"]).unwrap(), "2.00");
}

#[test]
fn test_f_large_value_not_truncated() {
    assert_eq!(format("{0:F2}", &["1234567.891"]).unwrap(), "1234567.89");
}

#[test]
fn test_f_with_alignment() {
    assert_eq!(format("{0,10:F2}", &["3.14159"]).unwrap(), "      3.14");
}

#[test]
fn test_f_rejects_non_number() {
    assert!(format("{0:F2}", &["abc"]).unwrap_err().contains("number"));
    assert!(format("{0:F2}", &["inf"]).unwrap_err().contains("number"));
}

// ===== N (number with thousands separators) =====

#[test]
fn test_n_groups_thousands() {
    assert_eq!(format("{0:N}", &["1234567.891"]).unwrap(), "1,234,567.89");
    assert_eq!(format("{0:N0}", &["1234567"]).unwrap(), "1,234,567");
    assert_eq!(format("{0:N3}", &["1000"]).unwrap(), "1,000.000");
}

#[test]
fn test_n_small_and_negative() {
    assert_eq!(format("{0:N2}", &["999"]).unwrap(), "999.00");
    assert_eq!(format("{0:N0}", &["-1234567"]).unwrap(), "-1,234,567");
}

// ===== C (currency) =====

#[test]
fn test_c_uses_culture_symbol() {
    assert_eq!(format("{0:C}", &["1234.5"]).unwrap(), "£1,234.50");
    assert_eq!(
        format_in(Culture::EnUs, "{0:C0}", &["1234.5"]).unwrap(),
        "$1,235"
    );
}

#[test]
fn test_c_uses_overridden_symbol() {
    let mut number_format = Culture::EnGb.number_format();
    number_format.currency_symbol = "GBP ".to_string();
    assert_eq!(
        apply_csharp_format("{0:C}", &args(&["1234.5"]), &number_format).unwrap(),
        "GBP 1,234.50"
    );
}

#[test]
fn test_c_negative_puts_sign_before_symbol() {
    assert_eq!(format("{0:C}", &["-1234.5"]).unwrap(), "-£1,234.50");
}

// ===== X (hexadecimal) =====

#[test]
fn test_x_case_follows_specifier() {
    assert_eq!(format("{0:X}", &["255"]).unwrap(), "FF");
    assert_eq!(format("{0:x}", &["255"]).unwrap(), "ff");
}

#[test]
fn test_x_zero_pads() {
    assert_eq!(format("{0:X4}", &["255"]).unwrap(), "00FF");
    assert_eq!(format("{0:x8}", &["48879"]).unwrap(), "0000beef");
}

#[test]
fn test_x_negative_is_twos_complement() {
    assert_eq!(format("{0:X}", &["-1"]).unwrap(), "FFFFFFFFFFFFFFFF");
}

#[test]
fn test_x_rejects_non_integer() {
    assert!(format("{0:X}", &["ff"]).unwrap_err().contains("integer"));
}

// ===== E (exponential) =====

#[test]
fn test_e_format() {
    assert_eq!(format("{0:E3}", &["3.14159"]).unwrap(), "3.142E+000");
    assert_eq!(format("{0:e2}", &["12345"]).unwrap(), "1.23e+004");
    assert_eq!(format("{0:E}", &["0.00015"]).unwrap(), "1.500000E-004");
}

// ===== G (general) =====

#[test]
fn test_g_shortest_form() {
    assert_eq!(format("{0:G}", &["3.14159"]).unwrap(), "3.14159");
    assert_eq!(format("{0:G}", &["42"]).unwrap(), "42");
    assert_eq!(format("{0:G}", &["0.00001"]).unwrap(), "1E-05");
}

#[test]
fn test_g_with_precision() {
    assert_eq!(format("{0:G3}", &["3.14159"]).unwrap(), "3.14");
    assert_eq!(format("{0:G2}", &["12345"]).unwrap(), "1.2E+04");
    assert_eq!(format("{0:g2}", &["12345"]).unwrap(), "1.2e+04");
    assert_eq!(format("{0:G4}", &["2.5"]).unwrap(), "2.5");
}

// ===== Cultures =====

#[test]
fn test_de_de_number_and_currency() {
    assert_eq!(
        format_in(Culture::DeDe, "{0:N2}", &["1234567.891"]).unwrap(),
        "1.234.567,89"
    );
    assert_eq!(
        format_in(Culture::DeDe, "{0:C}", &["1234.5"]).unwrap(),
        "1.234,50 €"
    );
    assert_eq!(
        format_in(Culture::DeDe, "{0:C}", &["-1234.5"]).unwrap(),
        "-1.234,50 €"
    );
}

#[test]
fn test_fr_fr_number_and_currency() {
    assert_eq!(
        format_in(Culture::FrFr, "{0:N2}", &["1234567.891"]).unwrap(),
        "1 234 567,89"
    );
    assert_eq!(
        format_in(Culture::FrFr, "{0:C}", &["1234.5"]).unwrap(),
        "1 234,50 €"
    );
}

#[test]
fn test_en_us_currency() {
    assert_eq!(
        format_in(Culture::EnUs, "{0:C}", &["-1234.5"]).unwrap(),
        "-$1,234.50"
    );
}

#[test]
fn test_culture_decimal_separator_in_f_e_and_g() {
    assert_eq!(
        format_in(Culture::DeDe, "{0:F2}", &["3.14159"]).unwrap(),
        "3,14"
    );
    assert_eq!(
        format_in(Culture::DeDe, "{0:E3}", &["3.14159"]).unwrap(),
        "3,142E+000"
    );
    assert_eq!(
        format_in(Culture::DeDe, "{0:G}", &["3.14159"]).unwrap(),
        "3,14159"
    );
}

#[test]
fn test_culture_does_not_change_d_or_x() {
    assert_eq!(
        format_in(Culture::DeDe, "{0:D6}", &["1234"]).unwrap(),
        "001234"
    );
    assert_eq!(
        format_in(Culture::DeDe, "{0:X}", &["4660"]).unwrap(),
        "1234"
    );
}

#[test]
fn test_f_does_not_group_thousands() {
    assert_eq!(
        format_in(Culture::DeDe, "{0:F1}", &["1234567.25"]).unwrap(),
        "1234567,3"
    );
}

#[test]
fn test_separator_overrides() {
    let mut number_format = Culture::EnGb.number_format();
    number_format.decimal_separator = ",".to_string();
    number_format.group_separator = "'".to_string();
    assert_eq!(
        apply_csharp_format("{0:N2}", &args(&["1234567.891"]), &number_format).unwrap(),
        "1'234'567,89"
    );
}
