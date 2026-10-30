//! The Rust `{}` format style, with automatic and explicit argument indexes.

use super::braces::{BraceToken, apply_indexed_format, brace_tokens};

fn preprocess_format(format_str: &str) -> Result<(String, usize), String> {
    let mut result = String::new();
    let mut auto_index = 0usize;

    for token in brace_tokens(format_str) {
        match token? {
            BraceToken::Literal(ch) => result.push(ch),
            BraceToken::EscapedOpen => result.push_str("{{"),
            BraceToken::EscapedClose => result.push_str("}}"),
            BraceToken::Placeholder(content) => {
                if content.is_empty() {
                    result.push_str(&format!("{{{}}}", auto_index));
                    auto_index += 1;
                } else if content.starts_with(':') {
                    result.push_str(&format!("{{{}{}}}", auto_index, content));
                    auto_index += 1;
                } else {
                    result.push_str(&format!("{{{}}}", content));
                }
            }
        }
    }
    Ok((result, auto_index))
}

pub(crate) fn apply_format(format_str: &str, args: &[String]) -> Result<String, String> {
    let (processed, _) = preprocess_format(format_str)?;
    apply_indexed_format(&processed, args)
}

#[cfg(test)]
#[path = "rust_tests.rs"]
mod tests;
