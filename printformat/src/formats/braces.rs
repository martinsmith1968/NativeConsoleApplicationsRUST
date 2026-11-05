//! Tokenizing and applying `{}`-style format strings (shared by the Rust and C# formats).

use std::collections::HashMap;
use strfmt::strfmt;

/// A piece of a `{}`-style format string.
#[derive(Debug, PartialEq)]
pub(super) enum BraceToken {
    /// An ordinary character outside any placeholder.
    Literal(char),
    /// An escaped `{{`.
    EscapedOpen,
    /// An escaped `}}`.
    EscapedClose,
    /// The text between `{` and `}` (without the braces).
    Placeholder(String),
}

/// Splits a `{}`-style format string into tokens. Yields an error for an
/// unclosed `{` or a single `}`, and stops after the first error.
pub(super) struct BraceTokens<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    failed: bool,
}

pub(super) fn brace_tokens(format_str: &str) -> BraceTokens<'_> {
    BraceTokens {
        chars: format_str.chars().peekable(),
        failed: false,
    }
}

impl Iterator for BraceTokens<'_> {
    type Item = Result<BraceToken, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }

        let token = match self.chars.next()? {
            '{' if self.chars.peek() == Some(&'{') => {
                self.chars.next();
                Ok(BraceToken::EscapedOpen)
            }
            '{' => {
                let mut content = String::new();
                let mut closed = false;
                for inner in self.chars.by_ref() {
                    if inner == '}' {
                        closed = true;
                        break;
                    }
                    content.push(inner);
                }
                if closed {
                    Ok(BraceToken::Placeholder(content))
                } else {
                    Err("unclosed `{` in format string".to_string())
                }
            }
            '}' if self.chars.peek() == Some(&'}') => {
                self.chars.next();
                Ok(BraceToken::EscapedClose)
            }
            '}' => Err("single `}` in format string".to_string()),
            ch => Ok(BraceToken::Literal(ch)),
        };

        self.failed = token.is_err();
        Some(token)
    }
}

pub(super) fn apply_indexed_format(format_str: &str, args: &[String]) -> Result<String, String> {
    let required_count = count_required_args(format_str)?;

    if required_count != args.len() {
        return Err(format!(
            "format string has {} placeholder(s) but {} argument(s) were provided",
            required_count,
            args.len()
        ));
    }

    let vars: HashMap<String, &str> = args
        .iter()
        .enumerate()
        .map(|(i, s)| (i.to_string(), s.as_str()))
        .collect();

    strfmt(format_str, &vars).map_err(|e| format!("format error: {}", e))
}

fn count_required_args(format_str: &str) -> Result<usize, String> {
    let mut required_count = 0usize;

    for token in brace_tokens(format_str) {
        if let BraceToken::Placeholder(content) = token? {
            let placeholder = content
                .split([',', ':'])
                .next()
                .ok_or_else(|| "placeholder index missing in format string".to_string())?
                .trim();
            let index = placeholder.parse::<usize>().map_err(|_| {
                format!(
                    "invalid placeholder index `{}` in format string",
                    placeholder
                )
            })?;
            required_count = required_count.max(index + 1);
        }
    }

    Ok(required_count)
}

#[cfg(test)]
#[path = "braces_tests.rs"]
mod tests;
