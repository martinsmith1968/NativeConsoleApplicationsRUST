//! The C# composite format style (`{0,-10:F2}`), rendered directly from the arguments.
//!
//! Supports the standard numeric format specifiers C, D, E, F, G, N and X (any case,
//! with an optional precision), plus alignment. Separators and the currency layout
//! come from the caller's [`NumberFormat`]; arguments are always parsed with `.` as
//! the decimal point.

use super::braces::{BraceToken, brace_tokens};
use super::culture::{CurrencyPosition, NumberFormat};

/// The largest precision accepted in a format specifier such as `F2`.
const MAX_PRECISION: usize = 99;

/// A parsed placeholder: `{index[,alignment][:format]}`.
#[derive(Debug, PartialEq)]
struct Placeholder {
    index: usize,
    alignment: Option<i32>,
    format: Option<FormatSpec>,
}

/// A standard numeric format specifier, such as `F2` or `x8`.
#[derive(Debug, PartialEq)]
struct FormatSpec {
    /// The specifier letter as written (its case matters for `E`, `G` and `X`).
    kind: char,
    precision: Option<usize>,
}

/// A piece of a parsed C# format string.
enum Piece {
    Text(String),
    Placeholder(Placeholder),
}

/// Formats `args` using a C# composite format string.
pub(crate) fn apply_csharp_format(
    format_str: &str,
    args: &[String],
    number_format: &NumberFormat,
) -> Result<String, String> {
    let pieces = parse_format(format_str)?;

    let required_count = pieces
        .iter()
        .filter_map(|piece| match piece {
            Piece::Placeholder(placeholder) => Some(placeholder.index + 1),
            Piece::Text(_) => None,
        })
        .max()
        .unwrap_or(0);

    if required_count != args.len() {
        return Err(format!(
            "format string has {} placeholder(s) but {} argument(s) were provided",
            required_count,
            args.len()
        ));
    }

    let mut output = String::new();
    for piece in pieces {
        match piece {
            Piece::Text(text) => output.push_str(&text),
            Piece::Placeholder(placeholder) => {
                let arg = &args[placeholder.index];
                let value = match &placeholder.format {
                    Some(spec) => format_value(arg, spec, number_format)?,
                    None => arg.clone(),
                };
                output.push_str(&align(&value, placeholder.alignment));
            }
        }
    }

    Ok(output)
}

fn parse_format(format_str: &str) -> Result<Vec<Piece>, String> {
    let mut pieces = Vec::new();
    let mut text = String::new();

    for token in brace_tokens(format_str) {
        match token? {
            BraceToken::Literal(ch) => text.push(ch),
            BraceToken::EscapedOpen => text.push('{'),
            BraceToken::EscapedClose => text.push('}'),
            BraceToken::Placeholder(content) => {
                pieces.push(Piece::Text(std::mem::take(&mut text)));
                pieces.push(Piece::Placeholder(parse_placeholder(&content)?));
            }
        }
    }
    pieces.push(Piece::Text(text));

    Ok(pieces)
}

fn parse_placeholder(content: &str) -> Result<Placeholder, String> {
    let invalid = || format!("invalid C# placeholder `{{{}}}`", content);

    let trimmed = content.trim();
    let index_end = trimmed
        .find(|ch: char| !ch.is_ascii_digit())
        .unwrap_or(trimmed.len());

    if index_end == 0 {
        return Err(invalid());
    }

    let index = trimmed[..index_end]
        .parse::<usize>()
        .map_err(|_| invalid())?;
    let mut rest = trimmed[index_end..].trim_start();

    let alignment = if let Some(remainder) = rest.strip_prefix(',') {
        let remainder = remainder.trim_start();
        let digits_start = if remainder.starts_with('-') || remainder.starts_with('+') {
            1
        } else {
            0
        };
        let digits_len = remainder[digits_start..]
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .count();
        if digits_len == 0 {
            return Err(format!("invalid C# alignment in `{{{}}}`", content));
        }

        let value = remainder[..digits_start + digits_len]
            .parse::<i32>()
            .map_err(|_| format!("invalid C# alignment in `{{{}}}`", content))?;
        rest = remainder[digits_start + digits_len..].trim_start();
        Some(value)
    } else {
        None
    };

    let format = if let Some(remainder) = rest.strip_prefix(':') {
        rest = "";
        parse_format_spec(remainder.trim())?
    } else {
        None
    };

    if !rest.is_empty() {
        return Err(invalid());
    }

    Ok(Placeholder {
        index,
        alignment,
        format,
    })
}

fn parse_format_spec(spec: &str) -> Result<Option<FormatSpec>, String> {
    let mut chars = spec.chars();
    let Some(kind) = chars.next() else {
        return Ok(None);
    };
    let digits = chars.as_str();

    let supported = matches!(
        kind.to_ascii_uppercase(),
        'C' | 'D' | 'E' | 'F' | 'G' | 'N' | 'X'
    );
    if !supported || !digits.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("C# format specifier '{}' is not supported", spec));
    }

    let precision = if digits.is_empty() {
        None
    } else {
        match digits.parse::<usize>() {
            Ok(precision) if precision <= MAX_PRECISION => Some(precision),
            _ => {
                return Err(format!(
                    "C# format precision in '{}' is too large (maximum {})",
                    spec, MAX_PRECISION
                ));
            }
        }
    };

    Ok(Some(FormatSpec { kind, precision }))
}

fn format_value(
    arg: &str,
    spec: &FormatSpec,
    number_format: &NumberFormat,
) -> Result<String, String> {
    let kind = spec.kind;

    Ok(match kind.to_ascii_uppercase() {
        'D' => {
            let value = parse_integer(arg, kind)?;
            let digits = zero_pad(&value.unsigned_abs().to_string(), spec.precision);
            if value < 0 {
                format!("-{}", digits)
            } else {
                digits
            }
        }
        'X' => {
            let value = parse_integer(arg, kind)?;
            let hex = if kind == 'x' {
                format!("{:x}", value)
            } else {
                format!("{:X}", value)
            };
            zero_pad(&hex, spec.precision)
        }
        'F' => {
            let value = parse_number(arg, kind)?;
            localize(
                &format_fixed(value, spec.precision.unwrap_or(2)),
                number_format,
                false,
            )
        }
        'N' => {
            let value = parse_number(arg, kind)?;
            localize(
                &format_fixed(value, spec.precision.unwrap_or(2)),
                number_format,
                true,
            )
        }
        'C' => {
            let value = parse_number(arg, kind)?;
            let number = localize(
                &format_fixed(value, spec.precision.unwrap_or(2)),
                number_format,
                true,
            );
            let (sign, number) = match number.strip_prefix('-') {
                Some(positive) => ("-", positive),
                None => ("", number.as_str()),
            };
            let symbol = &number_format.currency_symbol;
            match number_format.currency_position {
                CurrencyPosition::Before => format!("{}{}{}", sign, symbol, number),
                CurrencyPosition::After => format!("{}{}{}", sign, number, symbol),
                CurrencyPosition::BeforeSpace => format!("{}{} {}", sign, symbol, number),
                CurrencyPosition::AfterSpace => format!("{}{} {}", sign, number, symbol),
            }
        }
        'E' => {
            let value = parse_number(arg, kind)?;
            localize(
                &format_exponential(value, spec.precision.unwrap_or(6), kind),
                number_format,
                false,
            )
        }
        'G' => {
            let value = parse_number(arg, kind)?;
            localize(
                &format_general(value, spec.precision, kind),
                number_format,
                false,
            )
        }
        _ => return Err(format!("C# format specifier '{}' is not supported", kind)),
    })
}

fn parse_integer(arg: &str, kind: char) -> Result<i64, String> {
    arg.parse::<i64>().map_err(|_| {
        format!(
            "argument '{}' is not a valid integer for C# format '{}'",
            arg, kind
        )
    })
}

fn parse_number(arg: &str, kind: char) -> Result<f64, String> {
    match arg.parse::<f64>() {
        Ok(value) if value.is_finite() => Ok(value),
        _ => Err(format!(
            "argument '{}' is not a valid number for C# format '{}'",
            arg, kind
        )),
    }
}

/// Pads `digits` with leading zeros to at least `width` characters.
fn zero_pad(digits: &str, width: Option<usize>) -> String {
    format!("{:0>width$}", digits, width = width.unwrap_or(0))
}

/// Formats `value` with `precision` decimals, rounding halves away from zero as
/// .NET does (Rust's own formatting rounds exact halves to even).
fn format_fixed(value: f64, precision: usize) -> String {
    // 1100 decimals is enough to print any f64 exactly, so the rounding decision
    // is made on the true value rather than on an already-rounded string.
    let exact = format!("{:.1100}", value.abs());
    let point = exact
        .find('.')
        .expect("fixed-point formatting always contains '.'");
    let keep = point + 1 + precision;
    let round_up = exact.as_bytes()[keep] >= b'5';

    let mut number = if precision == 0 {
        exact[..point].to_string()
    } else {
        exact[..keep].to_string()
    };
    if round_up {
        number = increment_last_digit(&number);
    }

    if value.is_sign_negative() && value != 0.0 {
        format!("-{}", number)
    } else {
        number
    }
}

/// Adds one to the last digit of an unsigned decimal string, carrying as needed.
fn increment_last_digit(number: &str) -> String {
    let mut chars: Vec<char> = number.chars().collect();
    for i in (0..chars.len()).rev() {
        match chars[i] {
            '.' => continue,
            '9' => chars[i] = '0',
            digit => {
                chars[i] = char::from(digit as u8 + 1);
                return chars.into_iter().collect();
            }
        }
    }
    std::iter::once('1').chain(chars).collect()
}

/// Rewrites a number formatted with `.` as the decimal point using the culture's
/// decimal separator and, when `grouped`, its group separator between each group
/// of three digits before the decimal point.
fn localize(number: &str, number_format: &NumberFormat, grouped: bool) -> String {
    let (sign, unsigned) = match number.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", number),
    };
    let (integer, fraction) = match unsigned.split_once('.') {
        Some((integer, fraction)) => (integer, Some(fraction)),
        None => (unsigned, None),
    };

    let mut digits = String::new();
    for (i, ch) in integer.chars().enumerate() {
        if grouped && i > 0 && (integer.len() - i) % 3 == 0 {
            digits.push_str(&number_format.group_separator);
        }
        digits.push(ch);
    }

    match fraction {
        Some(fraction) => format!(
            "{}{}{}{}",
            sign, digits, number_format.decimal_separator, fraction
        ),
        None => format!("{}{}", sign, digits),
    }
}

/// Splits Rust's exponential output (`1.5e-7`) into mantissa and exponent.
fn split_exponential(formatted: &str) -> (&str, i32) {
    let (mantissa, exponent) = formatted
        .split_once('e')
        .expect("Rust exponential formatting always contains 'e'");
    let exponent = exponent
        .parse()
        .expect("Rust exponential formatting always has an integer exponent");
    (mantissa, exponent)
}

/// C# `E` format: `d.dddE+ddd`, with at least three exponent digits.
fn format_exponential(value: f64, precision: usize, kind: char) -> String {
    let formatted = format!("{:.*e}", precision, value);
    let (mantissa, exponent) = split_exponential(&formatted);
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{}{}{}{:03}", mantissa, kind, sign, exponent.abs())
}

/// C# `G` format: the shorter of fixed-point and scientific notation, using
/// `precision` significant digits (or the shortest round-trip form if none is given).
fn format_general(value: f64, precision: Option<usize>, kind: char) -> String {
    let (limit, formatted) = match precision {
        Some(precision) if precision > 0 => (precision, format!("{:.*e}", precision - 1, value)),
        _ => (15, format!("{:e}", value)),
    };
    let (mantissa, exponent) = split_exponential(&formatted);

    if exponent <= -5 || exponent >= limit as i32 {
        let marker = if kind == 'g' { 'e' } else { 'E' };
        let sign = if exponent < 0 { '-' } else { '+' };
        format!(
            "{}{}{}{:02}",
            trim_fraction_zeros(mantissa),
            marker,
            sign,
            exponent.abs()
        )
    } else {
        let rounded: f64 = formatted
            .parse()
            .expect("Rust exponential formatting always parses back");
        let digit_count = mantissa.chars().filter(|ch| ch.is_ascii_digit()).count() as i32;
        let decimals = (digit_count - 1 - exponent).max(0) as usize;
        trim_fraction_zeros(&format!("{:.*}", decimals, rounded))
    }
}

/// Removes trailing zeros after a decimal point, and the point itself if nothing is left.
fn trim_fraction_zeros(number: &str) -> String {
    if number.contains('.') {
        number
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        number.to_string()
    }
}

/// Pads `value` with spaces to the C# alignment width: right-aligned when
/// positive, left-aligned when negative.
fn align(value: &str, alignment: Option<i32>) -> String {
    match alignment {
        Some(width) if width < 0 => {
            format!("{:<width$}", value, width = width.unsigned_abs() as usize)
        }
        Some(width) => format!("{:>width$}", value, width = width as usize),
        None => value.to_string(),
    }
}

#[cfg(test)]
#[path = "csharp_tests.rs"]
mod tests;
