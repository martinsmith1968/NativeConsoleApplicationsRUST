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

// ===== Property Tests =====

mod properties {
    use super::*;
    use proptest::prelude::*;

    /// Writes the tokens back out as format-string text.
    fn render(tokens: &[BraceToken]) -> String {
        tokens
            .iter()
            .map(|token| match token {
                BraceToken::Literal(ch) => ch.to_string(),
                BraceToken::EscapedOpen => "{{".to_string(),
                BraceToken::EscapedClose => "}}".to_string(),
                BraceToken::Placeholder(content) => format!("{{{}}}", content),
            })
            .collect()
    }

    proptest! {
        #[test]
        fn valid_format_strings_round_trip_through_the_tokens(format_str in r"[a{}0-9:, ]{0,30}") {
            let tokens: Result<Vec<_>, _> = brace_tokens(&format_str).collect();
            if let Ok(tokens) = tokens {
                prop_assert_eq!(render(&tokens), format_str);
            }
        }

        #[test]
        fn tokenizing_stops_after_the_first_error(format_str in r"[a{}0-9:, ]{0,30}") {
            let tokens: Vec<_> = brace_tokens(&format_str).collect();
            let errors = tokens.iter().filter(|token| token.is_err()).count();
            prop_assert!(errors <= 1);
            if errors == 1 {
                prop_assert!(tokens.last().unwrap().is_err());
            }
        }

        #[test]
        fn text_without_braces_is_all_literals(text in r"[^{}]{0,40}") {
            let tokens: Vec<_> = brace_tokens(&text).collect::<Result<_, _>>().unwrap();
            let expected: Vec<_> = text.chars().map(BraceToken::Literal).collect();
            prop_assert_eq!(tokens, expected);
        }
    }
}
