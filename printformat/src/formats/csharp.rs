//! The C# composite format style (`{0,-10:F2}`), translated into the Rust style.

use super::braces::{BraceToken, apply_indexed_format, brace_tokens};

/// Translates a C# format string and applies the arguments to it.
pub(crate) fn apply_csharp_format(format_str: &str, args: &[String]) -> Result<String, String> {
    let translated = translate_csharp_format(format_str)?;
    apply_indexed_format(&translated, args)
}

pub(crate) fn translate_csharp_format(format_str: &str) -> Result<String, String> {
    let mut result = String::new();

    for token in brace_tokens(format_str) {
        match token? {
            BraceToken::Literal(ch) => result.push(ch),
            BraceToken::EscapedOpen => result.push_str("{{"),
            BraceToken::EscapedClose => result.push_str("}}"),
            BraceToken::Placeholder(content) => {
                result.push('{');
                result.push_str(&translate_csharp_placeholder(&content)?);
                result.push('}');
            }
        }
    }

    Ok(result)
}

fn translate_csharp_placeholder(content: &str) -> Result<String, String> {
    let trimmed = content.trim();
    let index_end = trimmed
        .find(|ch: char| !ch.is_ascii_digit())
        .unwrap_or(trimmed.len());

    if index_end == 0 {
        return Err(format!("invalid C# placeholder `{{{}}}`", content));
    }

    let index = &trimmed[..index_end];
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

    let format_spec = if let Some(remainder) = rest.strip_prefix(':') {
        rest = "";
        Some(translate_csharp_specifier(remainder.trim())?)
    } else {
        None
    };

    if !rest.is_empty() {
        return Err(format!("invalid C# placeholder `{{{}}}`", content));
    }

    let spec = combine_translated_spec(alignment, format_spec)?;
    Ok(match spec {
        Some(spec) => format!("{}:{}", index, spec),
        None => index.to_string(),
    })
}

fn translate_csharp_specifier(spec: &str) -> Result<String, String> {
    if spec.is_empty() {
        return Ok(String::new());
    }

    let mut chars = spec.chars();
    let kind = chars.next().unwrap();
    let precision = chars.as_str();
    let upper_kind = kind.to_ascii_uppercase();

    if !precision.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("C# format specifier '{}' is not supported", spec));
    }

    match upper_kind {
        'D' => Ok(if precision.is_empty() {
            String::new()
        } else {
            format!("0>{}", precision)
        }),
        'F' => Ok(if precision.is_empty() {
            ".2".to_string()
        } else {
            format!(".{}", precision)
        }),
        'E' => Ok(if precision.is_empty() {
            kind.to_string()
        } else {
            format!(".{}{}", precision, kind)
        }),
        'G' => Ok(spec.to_string()),
        'X' => Err("C# format specifier 'X' (hex) is not supported".to_string()),
        'N' => Err("C# format specifier 'N' (thousands separator) is not supported".to_string()),
        'C' => Err("C# format specifier 'C' (currency) is not supported".to_string()),
        _ => Err(format!("C# format specifier '{}' is not supported", spec)),
    }
}

fn combine_translated_spec(
    alignment: Option<i32>,
    format_spec: Option<String>,
) -> Result<Option<String>, String> {
    let alignment_spec = alignment.map(|value| {
        if value < 0 {
            format!("<{}", -value)
        } else {
            format!(">{}", value)
        }
    });

    match (alignment_spec, format_spec) {
        (None, None) => Ok(None),
        (Some(alignment), None) => Ok(Some(alignment)),
        (None, Some(spec)) if spec.is_empty() => Ok(None),
        (None, Some(spec)) => Ok(Some(spec)),
        (Some(alignment), Some(spec)) if spec.is_empty() => Ok(Some(alignment)),
        (Some(_), Some(spec)) if spec.starts_with("0>") => {
            Err("combining C# alignment with zero-padding is not supported".to_string())
        }
        (Some(alignment), Some(spec)) => Ok(Some(format!("{}{}", alignment, spec))),
    }
}

#[cfg(test)]
#[path = "csharp_tests.rs"]
mod tests;
