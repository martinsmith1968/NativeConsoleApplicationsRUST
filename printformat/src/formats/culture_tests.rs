use super::*;
use std::path::PathBuf;

/// Writes `contents` to a uniquely named file in the temp folder and returns its path.
fn temp_culture_file(name: &str, contents: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "printformat-culture-{}-{}.toml",
        std::process::id(),
        name
    ));
    fs::write(&path, contents).unwrap();
    path
}

#[test]
fn test_resolve_built_in_ignores_case() {
    assert_eq!(
        resolve_culture("DE-de").unwrap(),
        Culture::DeDe.number_format()
    );
}

#[test]
fn test_resolve_unknown_name_lists_built_ins() {
    let error = resolve_culture("xx-XX").unwrap_err();
    assert!(error.contains("unknown culture 'xx-XX'"));
    assert!(error.contains("en-GB, en-US, de-DE, fr-FR"));
    assert!(error.contains("path to a culture file"));
}

#[test]
fn test_resolve_culture_file() {
    let path = temp_culture_file(
        "swiss",
        "base = \"de-DE\"\n\
         decimal_separator = \".\"\n\
         group_separator = \"’\"\n\
         currency_symbol = \"CHF\"\n\
         currency_position = \"before-space\"\n",
    );
    let number_format = resolve_culture(path.to_str().unwrap()).unwrap();
    fs::remove_file(&path).unwrap();

    assert_eq!(
        number_format,
        NumberFormat {
            decimal_separator: ".".to_string(),
            group_separator: "’".to_string(),
            currency_symbol: "CHF".to_string(),
            currency_position: CurrencyPosition::BeforeSpace,
        }
    );
}

#[test]
fn test_partial_file_defaults_to_en_gb() {
    let number_format = parse_culture_file("currency_symbol = \"₹\"").unwrap();
    let mut expected = Culture::EnGb.number_format();
    expected.currency_symbol = "₹".to_string();
    assert_eq!(number_format, expected);
}

#[test]
fn test_partial_file_uses_base() {
    let number_format = parse_culture_file("base = \"fr-fr\"\ncurrency_symbol = \"CHF\"").unwrap();
    let mut expected = Culture::FrFr.number_format();
    expected.currency_symbol = "CHF".to_string();
    assert_eq!(number_format, expected);
}

#[test]
fn test_empty_file_is_en_gb() {
    assert_eq!(
        parse_culture_file("").unwrap(),
        Culture::EnGb.number_format()
    );
}

#[test]
fn test_unknown_field_is_an_error() {
    let error = parse_culture_file("decmial_separator = \",\"").unwrap_err();
    assert!(error.contains("decmial_separator"), "{}", error);
}

#[test]
fn test_unknown_position_is_an_error() {
    let error = parse_culture_file("currency_position = \"prefix\"").unwrap_err();
    assert!(error.contains("prefix"), "{}", error);
    assert!(error.contains("before-space"), "{}", error);
}

#[test]
fn test_unknown_base_is_an_error() {
    let error = parse_culture_file("base = \"xx-XX\"").unwrap_err();
    assert!(error.contains("unknown base culture 'xx-XX'"), "{}", error);
}

#[test]
fn test_empty_decimal_separator_is_an_error() {
    let error = parse_culture_file("decimal_separator = \"\"").unwrap_err();
    assert_eq!(error, "decimal_separator cannot be empty");
}

#[test]
fn test_invalid_toml_is_an_error_with_the_path() {
    let path = temp_culture_file("invalid", "this is not toml");
    let error = resolve_culture(path.to_str().unwrap()).unwrap_err();
    fs::remove_file(&path).unwrap();
    assert!(error.starts_with(path.to_str().unwrap()), "{}", error);
}

#[test]
fn test_error_reports_line_number() {
    let error =
        parse_culture_file("currency_symbol = \"X\"\ndecmial_separator = \",\"").unwrap_err();
    assert!(error.starts_with("line 2: "), "{}", error);
}
