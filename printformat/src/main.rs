use clap::Parser;
use std::collections::HashMap;
use strfmt::strfmt;

/// Format and print text using a format string and arguments
#[derive(Parser, Debug)]
#[command(
    version,
    bin_name = env!("CARGO_PKG_NAME"),
    about = concat!("printformat v", env!("CARGO_PKG_VERSION"), " - Format and print text using a format string and arguments\nCopyright © 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    long_about = concat!("printformat v", env!("CARGO_PKG_VERSION"), " - Format and print text using a format string and arguments\nCopyright © 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    author,
    help_expected = true,
    disable_help_flag = true,
    disable_version_flag = true,
    after_help = "Examples:\n  printformat \"Hello, {}!\" \"World\"\n  printformat \"{} + {} = {}\" \"1\" \"2\" \"3\"\n  printformat \"{:>10}\" \"right\"\n  printformat \"{:<10}\" \"left\"\n  printformat \"{:*^20}\" \"center\"\n  printformat \"No placeholders\""
)]
struct Args {
    /// The format string (use {} as placeholders)
    #[arg(index = 1, required = true)]
    format_string: String,

    /// Arguments to substitute into the format string
    #[arg(index = 2, num_args(0..))]
    arguments: Vec<String>,

    /// Print help
    #[arg(short = 'h', long, visible_short_alias = '?', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, visible_short_alias = '!', action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn main() {
    let args = Args::parse();

    match apply_format(&args.format_string, &args.arguments) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn preprocess_format(format_str: &str) -> Result<(String, usize), String> {
    let mut result = String::new();
    let mut chars = format_str.chars().peekable();
    let mut auto_index = 0usize;

    while let Some(ch) = chars.next() {
        if ch == '{' {
            match chars.peek() {
                Some(&'{') => {
                    chars.next();
                    result.push_str("{{");
                }
                _ => {
                    let mut content = String::new();
                    let mut closed = false;
                    for inner in chars.by_ref() {
                        if inner == '}' {
                            closed = true;
                            break;
                        }
                        content.push(inner);
                    }
                    if !closed {
                        return Err("unclosed `{` in format string".to_string());
                    }
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
        } else if ch == '}' {
            match chars.peek() {
                Some(&'}') => {
                    chars.next();
                    result.push_str("}}");
                }
                _ => {
                    return Err("single `}` in format string".to_string());
                }
            }
        } else {
            result.push(ch);
        }
    }
    Ok((result, auto_index))
}

fn apply_format(format_str: &str, args: &[String]) -> Result<String, String> {
    let (processed, required_count) = preprocess_format(format_str)?;

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

    strfmt(&processed, &vars).map_err(|e| format!("format error: {}", e))
}

#[cfg(test)]
mod main_tests;
