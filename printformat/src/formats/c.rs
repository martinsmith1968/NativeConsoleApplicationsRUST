//! The C `printf` format style (`%-10s`, `%05d`, `%.2f`).

use sprintf::{Printf as SprintfPrintf, vsprintf};

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
                        return Err("dynamic precision (*) is not supported in C format strings"
                            .to_string());
                    }
                    while chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                        chars.next();
                    }
                }

                // Length modifier (skip — type is determined by specifier)
                while let Some(&('h' | 'l' | 'j' | 'z' | 't' | 'L')) = chars.peek() {
                    chars.next();
                }

                let spec = chars
                    .next()
                    .ok_or_else(|| "incomplete format specifier at end of string".to_string())?;

                match spec {
                    'd' | 'i' | 'u' | 'o' | 'x' | 'X' | 'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 's'
                    | 'c' => {
                        specifiers.push(spec);
                    }
                    'n' => return Err("format specifier '%n' is not supported".to_string()),
                    'p' => {
                        return Err("format specifier '%p' (pointer) is not supported".to_string());
                    }
                    'a' | 'A' => {
                        return Err(format!(
                            "format specifier '%{}' (hex float) is not supported",
                            spec
                        ));
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
            let n: i64 = s
                .parse()
                .map_err(|_| format!("argument '{}' is not a valid integer for %{}", s, spec))?;
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
            let f: f64 = s
                .parse()
                .map_err(|_| format!("argument '{}' is not a valid number for %{}", s, spec))?;
            Box::new(f) as Box<dyn SprintfPrintf>
        }
        's' => Box::new(s.to_string()) as Box<dyn SprintfPrintf>,
        'c' => {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => Box::new(c) as Box<dyn SprintfPrintf>,
                (None, _) => return Err("argument for %c cannot be empty".to_string()),
                (Some(_), Some(_)) => {
                    return Err(format!(
                        "argument '{}' for %c must be a single character",
                        s
                    ));
                }
            }
        }
        _ => return Err(format!("unsupported specifier '%{}'", spec)),
    })
}

#[cfg(test)]
#[path = "c_tests.rs"]
mod tests;
