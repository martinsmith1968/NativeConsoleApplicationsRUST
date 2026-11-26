//! Culture settings for the C# format type: number separators and currency layout.
//!
//! A culture is either one of the built-in names (`en-GB`, `en-US`, `de-DE`,
//! `fr-FR`) or the path to a TOML culture file, for example:
//!
//! ```toml
//! base = "de-DE"                  # optional: start from a built-in culture (default en-GB)
//! decimal_separator = "."
//! group_separator = "’"
//! currency_symbol = "CHF"
//! currency_position = "before-space"
//! ```

use clap::ValueEnum;
use serde::Deserialize;
use std::fs;

/// The cultures with built-in number formats.
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Culture {
    #[default]
    #[value(name = "en-GB")]
    EnGb,
    #[value(name = "en-US")]
    EnUs,
    #[value(name = "de-DE")]
    DeDe,
    #[value(name = "fr-FR")]
    FrFr,
}

/// Where the currency symbol goes relative to the number.
#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum CurrencyPosition {
    /// `£1,234.50`
    Before,
    /// `1,234.50€`
    After,
    /// `CHF 1,234.50`
    BeforeSpace,
    /// `1.234,50 €`
    AfterSpace,
}

/// The separators and currency layout used to format numbers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NumberFormat {
    pub(crate) decimal_separator: String,
    pub(crate) group_separator: String,
    pub(crate) currency_symbol: String,
    pub(crate) currency_position: CurrencyPosition,
}

/// The contents of a culture file. Every field is optional; missing ones come
/// from `base`.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
struct CultureFile {
    base: Option<String>,
    decimal_separator: Option<String>,
    group_separator: Option<String>,
    currency_symbol: Option<String>,
    currency_position: Option<CurrencyPosition>,
}

impl Culture {
    /// Looks up a built-in culture by name, ignoring case.
    pub(crate) fn from_name(name: &str) -> Option<Culture> {
        <Culture as ValueEnum>::from_str(name, true).ok()
    }

    /// The culture's number format. fr-FR uses plain spaces where .NET uses
    /// non-breaking ones, so the output is easy to read and to process.
    pub(crate) fn number_format(self) -> NumberFormat {
        let (decimal_separator, group_separator, currency_symbol, currency_position) = match self {
            Culture::EnGb => (".", ",", "£", CurrencyPosition::Before),
            Culture::EnUs => (".", ",", "$", CurrencyPosition::Before),
            Culture::DeDe => (",", ".", "€", CurrencyPosition::AfterSpace),
            Culture::FrFr => (",", " ", "€", CurrencyPosition::AfterSpace),
        };

        NumberFormat {
            decimal_separator: decimal_separator.to_string(),
            group_separator: group_separator.to_string(),
            currency_symbol: currency_symbol.to_string(),
            currency_position,
        }
    }
}

/// The built-in culture names, for error messages.
fn built_in_names() -> String {
    Culture::value_variants()
        .iter()
        .filter_map(|culture| culture.to_possible_value())
        .map(|value| value.get_name().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Resolves a `--culture` value: a built-in culture name (checked first, ignoring
/// case) or the path to a culture file.
pub(crate) fn resolve_culture(value: &str) -> Result<NumberFormat, String> {
    if let Some(culture) = Culture::from_name(value) {
        return Ok(culture.number_format());
    }

    let contents = fs::read_to_string(value).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            format!(
                "unknown culture '{}' (expected one of {}, or the path to a culture file)",
                value,
                built_in_names()
            )
        } else {
            format!("{}: {}", value, error)
        }
    })?;

    parse_culture_file(&contents).map_err(|error| format!("{}: {}", value, error))
}

/// Builds a number format from the contents of a culture file.
fn parse_culture_file(contents: &str) -> Result<NumberFormat, String> {
    let file: CultureFile = toml::from_str(contents).map_err(|error| {
        // Report the line rather than toml's multi-line snippet, to keep errors on one line
        match error.span() {
            Some(span) => format!(
                "line {}: {}",
                contents[..span.start].matches('\n').count() + 1,
                error.message()
            ),
            None => error.message().to_string(),
        }
    })?;

    let base = match &file.base {
        Some(name) => Culture::from_name(name).ok_or_else(|| {
            format!(
                "unknown base culture '{}' (expected one of {})",
                name,
                built_in_names()
            )
        })?,
        None => Culture::default(),
    };

    let mut number_format = base.number_format();
    if let Some(separator) = file.decimal_separator {
        if separator.is_empty() {
            return Err("decimal_separator cannot be empty".to_string());
        }
        number_format.decimal_separator = separator;
    }
    if let Some(separator) = file.group_separator {
        number_format.group_separator = separator;
    }
    if let Some(symbol) = file.currency_symbol {
        number_format.currency_symbol = symbol;
    }
    if let Some(position) = file.currency_position {
        number_format.currency_position = position;
    }

    Ok(number_format)
}

#[cfg(test)]
#[path = "culture_tests.rs"]
mod tests;
