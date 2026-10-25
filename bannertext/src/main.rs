use clap::Parser;
use std::io::{self, Write};
use std::process::ExitCode;

/// The upper bound for the number of header or footer lines
const MAX_LINE_COUNT: u32 = 100;

/// The upper bound for prefix/suffix character counts and gap sizes
const MAX_AFFIX_SIZE: u32 = 1_000;

/// The upper bound for the banner width
const MAX_TOTAL_LENGTH: u32 = 10_000;

fn parse_single_char(s: &str) -> Result<char, String> {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Ok(c),
        _ => Err(format!("'{}' must be a single character", s)),
    }
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq, Copy, Default)]
#[clap(rename_all = "PascalCase")]
pub enum TextAlignment {
    #[default]
    Left,
    Right,
    Center,
}

impl std::fmt::Display for TextAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            TextAlignment::Left => "Left",
            TextAlignment::Right => "Right",
            TextAlignment::Center => "Center",
        };
        f.write_str(name)
    }
}

/// Display a Text Banner in the console
#[derive(Parser, Debug)]
#[command(
    version,
    bin_name = env!("CARGO_PKG_NAME"),
    about = concat!("bannertext v", env!("CARGO_PKG_VERSION"), " - Display a Text Banner in the console\nCopyright \u{00A9} 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    long_about = concat!("bannertext v", env!("CARGO_PKG_VERSION"), " - Display a Text Banner in the console\nCopyright \u{00A9} 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    author,
    help_expected = true,
    disable_help_flag = true,
    disable_version_flag = true,
    after_help = "Examples:\n  bannertext \"Hello World\"\n  bannertext \"Hello World\" --min-total-length 80\n  bannertext \"Hello World\" --min-total-length 80 --text-alignment Center\n  bannertext \"Hello World\" --header-line-char '#' --footer-line-char '#' --text-line-char '#'"
)]
struct Args {
    /// The Text to display (one or more values)
    #[arg(num_args(1..), required = true)]
    message_text: Vec<String>,

    /// The character to use for header lines
    #[arg(short = 'H', long, default_value = "*", value_parser = parse_single_char)]
    header_line_char: char,

    /// The number of header lines to print
    #[arg(
        short = 'n',
        long,
        default_value_t = 1,
        value_parser = clap::value_parser!(u32).range(0..=MAX_LINE_COUNT as i64)
    )]
    header_line_count: u32,

    /// The character to use for footer lines
    #[arg(short = 'F', long, default_value = "*", value_parser = parse_single_char)]
    footer_line_char: char,

    /// The number of footer lines to print
    #[arg(
        short = 'N',
        long,
        default_value_t = 1,
        value_parser = clap::value_parser!(u32).range(0..=MAX_LINE_COUNT as i64)
    )]
    footer_line_count: u32,

    /// The character to use for text line prefix/suffix
    #[arg(short = 'L', long, default_value = "*", value_parser = parse_single_char)]
    text_line_char: char,

    /// Set Title Prefix Count
    #[arg(
        short = 'p',
        long,
        default_value_t = 2,
        value_parser = clap::value_parser!(u32).range(0..=MAX_AFFIX_SIZE as i64)
    )]
    title_prefix_count: u32,

    /// Set Title Suffix Count
    #[arg(
        short = 's',
        long,
        default_value_t = 2,
        value_parser = clap::value_parser!(u32).range(0..=MAX_AFFIX_SIZE as i64)
    )]
    title_suffix_count: u32,

    /// Set Title Prefix Gap Size
    #[arg(
        short = 'P',
        long,
        default_value_t = 2,
        value_parser = clap::value_parser!(u32).range(0..=MAX_AFFIX_SIZE as i64)
    )]
    title_prefix_gap_size: u32,

    /// Set Title Suffix Gap Size
    #[arg(
        short = 'S',
        long,
        default_value_t = 2,
        value_parser = clap::value_parser!(u32).range(0..=MAX_AFFIX_SIZE as i64)
    )]
    title_suffix_gap_size: u32,

    /// Set Text Alignment
    #[arg(short = 'a', long, default_value_t = TextAlignment::Left)]
    text_alignment: TextAlignment,

    /// Set Minimum Total line length
    #[arg(
        short = 'm',
        long,
        default_value_t = 0,
        value_parser = clap::value_parser!(u32).range(0..=MAX_TOTAL_LENGTH as i64)
    )]
    min_total_length: u32,

    /// Set Maximum Total line length
    #[arg(
        short = 'M',
        long,
        default_value_t = 0,
        value_parser = clap::value_parser!(u32).range(0..=MAX_TOTAL_LENGTH as i64)
    )]
    max_total_length: u32,

    /// Print help
    #[arg(short = 'h', long, visible_short_alias = '?', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, visible_short_alias = '!', action = clap::ArgAction::Version)]
    version: Option<bool>,
}

/// Every setting that controls the generated banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BannerOptions {
    pub header_line_char: char,
    pub header_line_count: u32,
    pub footer_line_char: char,
    pub footer_line_count: u32,
    pub text_line_char: char,
    pub title_prefix_count: u32,
    pub title_suffix_count: u32,
    pub title_prefix_gap_size: u32,
    pub title_suffix_gap_size: u32,
    pub text_alignment: TextAlignment,
    pub min_total_length: u32,
    pub max_total_length: u32,
}

impl Default for BannerOptions {
    fn default() -> Self {
        Self {
            header_line_char: '*',
            header_line_count: 1,
            footer_line_char: '*',
            footer_line_count: 1,
            text_line_char: '*',
            title_prefix_count: 2,
            title_suffix_count: 2,
            title_prefix_gap_size: 2,
            title_suffix_gap_size: 2,
            text_alignment: TextAlignment::Left,
            min_total_length: 0,
            max_total_length: 0,
        }
    }
}

impl BannerOptions {
    fn from_args(args: &Args) -> Self {
        Self {
            header_line_char: args.header_line_char,
            header_line_count: args.header_line_count,
            footer_line_char: args.footer_line_char,
            footer_line_count: args.footer_line_count,
            text_line_char: args.text_line_char,
            title_prefix_count: args.title_prefix_count,
            title_suffix_count: args.title_suffix_count,
            title_prefix_gap_size: args.title_prefix_gap_size,
            title_suffix_gap_size: args.title_suffix_gap_size,
            text_alignment: args.text_alignment,
            min_total_length: args.min_total_length,
            max_total_length: args.max_total_length,
        }
    }
}

pub fn generate_banner(texts: &[&str], options: &BannerOptions) -> String {
    let prefix_chars = options
        .text_line_char
        .to_string()
        .repeat(options.title_prefix_count as usize);
    let prefix_gap = " ".repeat(options.title_prefix_gap_size as usize);
    let suffix_gap = " ".repeat(options.title_suffix_gap_size as usize);
    let suffix_chars = options
        .text_line_char
        .to_string()
        .repeat(options.title_suffix_count as usize);

    // Use usize arithmetic with saturation so the sums cannot overflow
    let prefix_total = (options.title_prefix_count as usize)
        .saturating_add(options.title_prefix_gap_size as usize);
    let suffix_total = (options.title_suffix_count as usize)
        .saturating_add(options.title_suffix_gap_size as usize);

    let max_text_len = texts.iter().map(|t| t.chars().count()).max().unwrap_or(0);
    let natural_length = prefix_total
        .saturating_add(max_text_len)
        .saturating_add(suffix_total);

    let mut total_length = natural_length;
    if options.min_total_length > 0 {
        total_length = total_length.max(options.min_total_length as usize);
    }
    if options.max_total_length > 0 {
        total_length = total_length.min(options.max_total_length as usize);
    }

    let affix_total = prefix_total.saturating_add(suffix_total);
    let text_area_width = total_length.saturating_sub(affix_total);

    let header_line = options.header_line_char.to_string().repeat(total_length);
    let footer_line = options.footer_line_char.to_string().repeat(total_length);

    let mut lines: Vec<String> = Vec::new();

    for _ in 0..options.header_line_count {
        lines.push(header_line.clone());
    }

    for text in texts {
        let display_text: String = text.chars().take(text_area_width).collect();
        let formatted_text = match options.text_alignment {
            TextAlignment::Left => format!("{:<width$}", display_text, width = text_area_width),
            TextAlignment::Right => format!("{:>width$}", display_text, width = text_area_width),
            TextAlignment::Center => format!("{:^width$}", display_text, width = text_area_width),
        };
        lines.push(format!(
            "{}{}{}{}{}",
            prefix_chars, prefix_gap, formatted_text, suffix_gap, suffix_chars
        ));
    }

    for _ in 0..options.footer_line_count {
        lines.push(footer_line.clone());
    }

    lines.join("\n")
}

/// Validate relationships between arguments that clap cannot express on its own.
fn validate_args(args: &Args) -> Result<(), String> {
    if args.max_total_length > 0 && args.min_total_length > args.max_total_length {
        return Err(format!(
            "--min-total-length ({}) must not be greater than --max-total-length ({})",
            args.min_total_length, args.max_total_length
        ));
    }

    Ok(())
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
    validate_args(args).map_err(io::Error::other)?;

    let text_refs: Vec<&str> = args.message_text.iter().map(String::as_str).collect();
    let banner = generate_banner(&text_refs, &BannerOptions::from_args(args));

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", banner)
}

#[cfg(test)]
mod main_tests;
