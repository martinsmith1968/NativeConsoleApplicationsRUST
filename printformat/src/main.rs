use clap::Parser;

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
    after_help = "Examples:\n  printformat \"Hello, {}!\" \"World\"\n  printformat \"{} + {} = {}\" \"1\" \"2\" \"3\"\n  printformat \"No placeholders\""
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

fn apply_format(format_str: &str, args: &[String]) -> Result<String, String> {
    let placeholder_count = format_str.matches("{}").count();
    if placeholder_count != args.len() {
        return Err(format!(
            "format string has {} placeholder(s) but {} argument(s) were provided",
            placeholder_count,
            args.len()
        ));
    }

    let mut result = format_str.to_string();
    for arg in args {
        result = result.replacen("{}", arg, 1);
    }
    Ok(result)
}

#[cfg(test)]
mod main_tests;
