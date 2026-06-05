use clap::{Parser, ValueEnum};
use sprintf::{vsprintf, Printf as SprintfPrintf};
use std::collections::HashMap;
use strfmt::strfmt;

#[derive(ValueEnum, Clone, Debug, Default)]
enum FormatType {
    #[default]
    #[value(name = "Rust")]
    Rust,
    #[value(name = "CSharp")]
    CSharp,
    #[value(name = "C")]
    C,
}

impl std::fmt::Display for FormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatType::Rust => write!(f, "Rust"),
            FormatType::CSharp => write!(f, "CSharp"),
            FormatType::C => write!(f, "C"),
        }
    }
}

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
    after_help = "Examples:\n  printformat \"Hello, {}!\" \"World\"\n  printformat \"{} + {} = {}\" \"1\" \"2\" \"3\"\n  printformat \"{:>10}\" \"right\"\n  printformat \"{:<10}\" \"left\"\n  printformat \"{:*^20}\" \"center\"\n  printformat \"No placeholders\"\n  printformat --format-type CSharp \"{0} is {1} years old\" \"Alice\" \"30\"\n  printformat --format-type CSharp \"{0,-10} | {1,10}\" \"left\" \"right\"\n  printformat --format-type CSharp \"{0:D5}\" \"42\"\n  printformat --format-type C \"%s is %d years old\" \"Alice\" \"30\"\n  printformat --format-type C \"%10s | %-10s\" \"right\" \"left\"\n  printformat --format-type C \"%05d\" \"42\"\n  printformat --format-type C \"%.2f\" \"3.14159\"\n\nSee Also:\n https://docs.rs/strfmt/latest/strfmt/\n https://doc.rust-lang.org/std/fmt/\n https://cplusplus.com/reference/cstdio/printf/"
)]
struct Args {
    /// Format type to use for the format string
    #[arg(short = 't', long = "format-type", value_enum, default_value_t = FormatType::Rust)]
    format_type: FormatType,

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
    let result = match args.format_type {
        FormatType::CSharp => translate_csharp_format(&args.format_string)
            .and_then(|translated| apply_indexed_format(&translated, &args.arguments)),
        FormatType::C => apply_c_format(&args.format_string, &args.arguments),
        FormatType::Rust => apply_format(&args.format_string, &args.arguments),
    };

    match result {
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
    let (processed, _) = preprocess_format(format_str)?;
    apply_indexed_format(&processed, args)
}

fn apply_indexed_format(format_str: &str, args: &[String]) -> Result<String, String> {
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
    let mut chars = format_str.chars().peekable();
    let mut required_count = 0usize;

    while let Some(ch) = chars.next() {
        if ch == '{' {
            match chars.peek() {
                Some(&'{') => {
                    chars.next();
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
        } else if ch == '}' {
            match chars.peek() {
                Some(&'}') => {
                    chars.next();
                }
                _ => {
                    return Err("single `}` in format string".to_string());
                }
            }
        }
    }

    Ok(required_count)
}

pub(crate) fn translate_csharp_format(format_str: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = format_str.chars().peekable();

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
                    result.push('{');
                    result.push_str(&translate_csharp_placeholder(&content)?);
                    result.push('}');
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

pub(crate) fn apply_c_format(format_str: &str, args: &[String]) -> Result<String, String> {
    let specifiers = extract_c_specifiers(format_str)?;

    if specifiers.len() != args.len() {
        return Err(format!(
            "format string has {} placeholder(s) but {} argument(s) were provided",
            specifiers.len(),
            args.len()
        ));
    }

    let typed_args: Vec<Box<dyn SprintfPrintf>> = specifiers
        .iter()
        .zip(args.iter())
        .map(|(&spec, arg)| arg_to_printf(arg, spec))
        .collect::<Result<Vec<_>, _>>()?;

    let refs: Vec<&dyn SprintfPrintf> = typed_args.iter().map(|b| b.as_ref()).collect();

    vsprintf(format_str, &refs).map_err(|e| format!("format error: {}", e))
}

fn extract_c_specifiers(format_str: &str) -> Result<Vec<char>, String> {
    let mut specifiers = Vec::new();
    let mut chars = format_str.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '%' {
            continue;
        }

        match chars.peek() {
            None => return Err("incomplete format specifier at end of string".to_string()),
            Some(&'%') => {
                chars.next();
            }
            _ => {
                // Skip flags: -, +, space, #, 0
                while let Some(&flag) = chars.peek() {
                    if matches!(flag, '-' | '+' | ' ' | '#' | '0') {
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Width
                if chars.peek() == Some(&'*') {
                    return Err(
                        "dynamic width (*) is not supported in C format strings".to_string()
                    );
                }
                while chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    chars.next();
                }

                // Precision
                if chars.peek() == Some(&'.') {
                    chars.next();
                    if chars.peek() == Some(&'*') {
                        return Err(
                            "dynamic precision (*) is not supported in C format strings"
                                .to_string(),
                        );
                    }
                    while chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                        chars.next();
                    }
                }

                // Length modifier (skip — type is determined by specifier)
                loop {
                    match chars.peek() {
                        Some(&'h') | Some(&'l') | Some(&'j') | Some(&'z') | Some(&'t')
                        | Some(&'L') => {
                            chars.next();
                        }
                        _ => break,
                    }
                }

                let spec = chars.next().ok_or_else(|| {
                    "incomplete format specifier at end of string".to_string()
                })?;

                match spec {
                    'd' | 'i' | 'u' | 'o' | 'x' | 'X' | 'f' | 'F' | 'e' | 'E' | 'g' | 'G'
                    | 's' | 'c' => {
                        specifiers.push(spec);
                    }
                    'n' => {
                        return Err("format specifier '%n' is not supported".to_string())
                    }
                    'p' => {
                        return Err(
                            "format specifier '%p' (pointer) is not supported".to_string()
                        )
                    }
                    'a' | 'A' => {
                        return Err(format!(
                            "format specifier '%{}' (hex float) is not supported",
                            spec
                        ))
                    }
                    _ => return Err(format!("unknown format specifier '%{}'", spec)),
                }
            }
        }
    }

    Ok(specifiers)
}

fn arg_to_printf(s: &str, spec: char) -> Result<Box<dyn SprintfPrintf>, String> {
    Ok(match spec {
        'd' | 'i' => {
            let n: i64 = s.parse().map_err(|_| {
                format!("argument '{}' is not a valid integer for %{}", s, spec)
            })?;
            Box::new(n) as Box<dyn SprintfPrintf>
        }
        'u' | 'o' | 'x' | 'X' => {
            let n: u64 = s.parse().map_err(|_| {
                format!(
                    "argument '{}' is not a valid unsigned integer for %{}",
                    s, spec
                )
            })?;
            Box::new(n) as Box<dyn SprintfPrintf>
        }
        'f' | 'F' | 'e' | 'E' | 'g' | 'G' => {
            let f: f64 = s.parse().map_err(|_| {
                format!("argument '{}' is not a valid number for %{}", s, spec)
            })?;
            Box::new(f) as Box<dyn SprintfPrintf>
        }
        's' => Box::new(s.to_string()) as Box<dyn SprintfPrintf>,
        'c' => {
            let c = s
                .chars()
                .next()
                .ok_or_else(|| "argument for %c cannot be empty".to_string())?;
            Box::new(c) as Box<dyn SprintfPrintf>
        }
        _ => return Err(format!("unsupported specifier '%{}'", spec)),
    })
}

#[cfg(test)]
mod main_tests;
