use super::*;

fn tokens(format_str: &str) -> Vec<Result<BraceToken, String>> {
    brace_tokens(format_str).collect()
}

#[test]
fn test_brace_tokens_empty_string() {
    assert!(tokens("").is_empty());
}

#[test]
fn test_brace_tokens_literals_escapes_and_placeholders() {
    assert_eq!(
        tokens("a{{{0:>5}}}b"),
        vec![
            Ok(BraceToken::Literal('a')),
            Ok(BraceToken::EscapedOpen),
            Ok(BraceToken::Placeholder("0:>5".to_string())),
            Ok(BraceToken::EscapedClose),
            Ok(BraceToken::Literal('b')),
        ]
    );
}

#[test]
fn test_brace_tokens_empty_placeholder() {
    assert_eq!(
        tokens("{}"),
        vec![Ok(BraceToken::Placeholder(String::new()))]
    );
}

#[test]
fn test_brace_tokens_unclosed_brace_errors() {
    assert_eq!(
        tokens("x{0"),
        vec![
            Ok(BraceToken::Literal('x')),
            Err("unclosed `{` in format string".to_string()),
        ]
    );
}

#[test]
fn test_brace_tokens_single_close_brace_errors_and_stops() {
    assert_eq!(
        tokens("}{0}"),
        vec![Err("single `}` in format string".to_string())]
    );
}
