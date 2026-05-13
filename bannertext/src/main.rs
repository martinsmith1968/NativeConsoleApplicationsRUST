use clap::Parser;

fn parse_single_char(s: &str) -> Result<char, String> {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Ok(c),
        _ => Err(format!("'{}' must be a single character", s)),
    }
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Copy)]
#[clap(rename_all = "PascalCase")]
pub enum TextAlignment {
    Left,
    Right,
    Center,
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
    /// The Text to display
    #[arg()]
    message_text: String,

    /// The character to use for header lines
    #[arg(short = 'H', long, default_value = "*", value_parser = parse_single_char)]
    header_line_char: char,

    /// The number of header lines to print
    #[arg(short = 'n', long, default_value_t = 1)]
    header_line_count: u32,

    /// The character to use for footer lines
    #[arg(short = 'F', long, default_value = "*", value_parser = parse_single_char)]
    footer_line_char: char,

    /// The number of footer lines to print
    #[arg(short = 'N', long, default_value_t = 1)]
    footer_line_count: u32,

    /// The character to use for text line prefix/suffix
    #[arg(short = 'L', long, default_value = "*", value_parser = parse_single_char)]
    text_line_char: char,

    /// Set Title Prefix Count
    #[arg(short = 'p', long, default_value_t = 2)]
    title_prefix_count: u32,

    /// Set Title Suffix Count
    #[arg(short = 's', long, default_value_t = 2)]
    title_suffix_count: u32,

    /// Set Title Prefix Gap Size
    #[arg(short = 'P', long, default_value_t = 2)]
    title_prefix_gap_size: u32,

    /// Set Title Suffix Gap Size
    #[arg(short = 'S', long, default_value_t = 2)]
    title_suffix_gap_size: u32,

    /// Set Text Alignment
    #[arg(short = 'a', long, default_value = "Left")]
    text_alignment: TextAlignment,

    /// Set Minimum Total line length
    #[arg(short = 'm', long, default_value_t = 0)]
    min_total_length: u32,

    /// Set Maximum Total line length
    #[arg(short = 'M', long, default_value_t = 0)]
    max_total_length: u32,

    /// Print help
    #[arg(short = 'h', long, visible_short_alias = '?', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, visible_short_alias = '!', action = clap::ArgAction::Version)]
    version: Option<bool>,
}

pub fn generate_banner(
    text: &str,
    header_line_char: char,
    header_line_count: u32,
    footer_line_char: char,
    footer_line_count: u32,
    text_line_char: char,
    title_prefix_count: u32,
    title_suffix_count: u32,
    title_prefix_gap_size: u32,
    title_suffix_gap_size: u32,
    text_alignment: TextAlignment,
    min_total_length: u32,
    max_total_length: u32,
) -> String {
    let prefix_chars: String = std::iter::repeat(text_line_char)
        .take(title_prefix_count as usize)
        .collect();
    let prefix_gap: String = std::iter::repeat(' ')
        .take(title_prefix_gap_size as usize)
        .collect();
    let suffix_gap: String = std::iter::repeat(' ')
        .take(title_suffix_gap_size as usize)
        .collect();
    let suffix_chars: String = std::iter::repeat(text_line_char)
        .take(title_suffix_count as usize)
        .collect();

    let prefix_total = (title_prefix_count + title_prefix_gap_size) as usize;
    let suffix_total = (title_suffix_count + title_suffix_gap_size) as usize;
    let natural_length = prefix_total + text.chars().count() + suffix_total;

    let mut total_length = natural_length;
    if min_total_length > 0 {
        total_length = total_length.max(min_total_length as usize);
    }
    if max_total_length > 0 {
        total_length = total_length.min(max_total_length as usize);
    }

    let text_area_width = if total_length > prefix_total + suffix_total {
        total_length - prefix_total - suffix_total
    } else {
        0
    };

    // Truncate text if needed (handle multi-byte chars safely)
    let display_text: String = text.chars().take(text_area_width).collect();

    let formatted_text = match text_alignment {
        TextAlignment::Left => format!("{:<width$}", display_text, width = text_area_width),
        TextAlignment::Right => format!("{:>width$}", display_text, width = text_area_width),
        TextAlignment::Center => format!("{:^width$}", display_text, width = text_area_width),
    };

    let text_line = format!(
        "{}{}{}{}{}",
        prefix_chars, prefix_gap, formatted_text, suffix_gap, suffix_chars
    );

    let header_line: String = std::iter::repeat(header_line_char)
        .take(total_length)
        .collect();
    let footer_line: String = std::iter::repeat(footer_line_char)
        .take(total_length)
        .collect();

    let mut lines: Vec<String> = Vec::new();

    for _ in 0..header_line_count {
        lines.push(header_line.clone());
    }
    lines.push(text_line);
    for _ in 0..footer_line_count {
        lines.push(footer_line.clone());
    }

    lines.join("\n")
}

fn main() {
    let args = Args::parse();

    let banner = generate_banner(
        &args.message_text,
        args.header_line_char,
        args.header_line_count,
        args.footer_line_char,
        args.footer_line_count,
        args.text_line_char,
        args.title_prefix_count,
        args.title_suffix_count,
        args.title_prefix_gap_size,
        args.title_suffix_gap_size,
        args.text_alignment,
        args.min_total_length,
        args.max_total_length,
    );

    println!("{}", banner);
}

#[cfg(test)]
mod main_tests;
