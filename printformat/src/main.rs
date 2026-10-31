use clap::{Parser, ValueEnum};
use std::io::{self, Write};
use std::process::ExitCode;

mod formats;
use formats::c::apply_c_format;
use formats::csharp::apply_csharp_format;
use formats::culture::{Culture, NumberFormat};
use formats::rust::apply_format;

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

/// Format and print text using a format string and arguments
#[derive(Parser, Debug)]
#[command(
    version,
    bin_name = env!("CARGO_PKG_NAME"),
    about = concat!("printformat v", env!("CARGO_PKG_VERSION"), " - Format and print text using a format string and arguments\nCopyright © 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    long_about = concat!("printformat v", env!("CARGO_PKG_VERSION"), " - Format and print text using a format string and arguments\nCopyright © 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    author,
    help_expected = true,
    allow_negative_numbers = true,
    disable_help_flag = true,
    disable_version_flag = true,
    after_help = "Examples:\n  printformat \"Hello, {}!\" \"World\"\n  printformat \"{} + {} = {}\" \"1\" \"2\" \"3\"\n  printformat \"{:>10}\" \"right\"\n  printformat \"{:<10}\" \"left\"\n  printformat \"{:*^20}\" \"center\"\n  printformat \"No placeholders\"\n  printformat --format-type CSharp \"{0} is {1} years old\" \"Alice\" \"30\"\n  printformat --format-type CSharp \"{0,-10} | {1,10}\" \"left\" \"right\"\n  printformat --format-type CSharp \"{0:D5}\" \"42\"\n  printformat --format-type CSharp \"{0:N2}\" \"1234567.891\"\n  printformat --format-type CSharp \"{0:X4}\" \"255\"\n  printformat --format-type CSharp --currency \"$\" \"{0:C}\" \"1234.5\"\n  printformat --format-type CSharp --culture de-DE \"{0:N2} | {1:C}\" \"1234567.891\" \"1234.5\"\n  printformat --format-type C \"%s is %d years old\" \"Alice\" \"30\"\n  printformat --format-type C \"%10s | %-10s\" \"right\" \"left\"\n  printformat --format-type C \"%05d\" \"42\"\n  printformat --format-type C \"%.2f\" \"3.14159\"\n\nSee Also:\n https://docs.rs/strfmt/latest/strfmt/\n https://doc.rust-lang.org/std/fmt/\n https://cplusplus.com/reference/cstdio/printf/"
)]
struct Args {
    /// Format type to use for the format string
    #[arg(short = 't', long = "format-type", value_enum, default_value_t = FormatType::Rust)]
    format_type: FormatType,

    /// Culture whose separators and currency the C# format type uses
    #[arg(long = "culture", value_enum, ignore_case = true, default_value_t = Culture::EnGb)]
    culture: Culture,

    /// Currency symbol for the C# 'C' format specifier (overrides the culture's)
    #[arg(long = "currency")]
    currency: Option<String>,

    /// Decimal separator for C# number formats (overrides the culture's)
    #[arg(long = "decimal-separator")]
    decimal_separator: Option<String>,

    /// Group (thousands) separator for the C# 'N' and 'C' format specifiers (overrides the culture's)
    #[arg(long = "group-separator")]
    group_separator: Option<String>,

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

fn main() -> ExitCode {
    let args = Args::parse();

    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}

fn run(args: &Args) -> io::Result<()> {
    let result = match args.format_type {
        FormatType::CSharp => {
            apply_csharp_format(&args.format_string, &args.arguments, &number_format(args))
        }
        FormatType::C => apply_c_format(&args.format_string, &args.arguments),
        FormatType::Rust => apply_format(&args.format_string, &args.arguments),
    };

    match result {
        Ok(output) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            writeln!(handle, "{}", output)
        }
        Err(error) => Err(io::Error::other(error)),
    }
}

/// The culture's number format, with any separator or currency overrides applied.
fn number_format(args: &Args) -> NumberFormat {
    let mut number_format = args.culture.number_format();
    if let Some(currency) = &args.currency {
        number_format.currency_symbol = currency.clone();
    }
    if let Some(separator) = &args.decimal_separator {
        number_format.decimal_separator = separator.clone();
    }
    if let Some(separator) = &args.group_separator {
        number_format.group_separator = separator.clone();
    }
    number_format
}
