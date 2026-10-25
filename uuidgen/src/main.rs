use clap::{Parser, ValueEnum};
use nanoid::nanoid;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::ExitCode;
use strfmt::strfmt;
use uuid::Uuid;

// Notes:
// - https://medium.com/@mattpatchava/std-fmt-56c412e30d74

/// The node value used by a v6 GUID: six comma-separated byte values.
type UuidSeed = [u8; 6];

#[derive(ValueEnum, Clone, Debug, PartialEq, Copy)]
#[clap(rename_all = "kebab-case")]
enum GuidVersionType {
    V4,
    V6,
    V7,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Copy)]
#[clap(rename_all = "lowercase")]
enum UUIDType {
    Guid,
    NanoId,
}

/// Generate Unique IDs (UUIDs) with controlled output and formatting
#[derive(Parser, Debug)]
#[command(
    version,
    bin_name = env!("CARGO_PKG_NAME"),
    about = concat!("uuidgen v", env!("CARGO_PKG_VERSION"), " - Generate Unique IDs (UUIDs) with controlled output and formatting\nCopyright \u{00A9} 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    long_about = concat!("uuidgen v", env!("CARGO_PKG_VERSION"), " - Generate Unique IDs (UUIDs) with controlled output and formatting\nCopyright \u{00A9} 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    author,
    help_expected = true,
    disable_help_flag = true,
    disable_version_flag = true,
    after_help = "NOTE:\noutput-template supports: {uuid}, {sequence} dynamic values\n(See also : https://github.com/vitiral/strfmt)\n\nExamples:\n  uuidgen\n  uuidgen --count 5\n  uuidgen --count 3 --uppercase\n  uuidgen --uuid-type nanoid\n  uuidgen --count 5 --output-template \"{sequence}: {uuid}\""
)]
struct Args {
    /// Number of times to generate
    #[arg(short = 'c', long, default_value_t = 1)]
    count: u32,

    /// The type of UUID to generate
    #[arg(short = 't', long, default_value = "guid")]
    uuid_type: UUIDType,

    /// The version of GUID to generate
    #[arg(short = 'v', long, default_value = "v4")]
    guid_version: GuidVersionType,

    /// The size of NanoId to generate
    #[arg(short = 'l', long, default_value_t = 21)]
    nanoid_length: usize,

    /// Format the GUID without Hyphens (GUID only)
    #[arg(short = 'y', long, default_value_t = false)]
    non_hyphenated: bool,

    /// Covert the GUID to Upper case values (GUID only)
    #[arg(short = 'u', long, default_value_t = false)]
    uppercase: bool,

    /// The template to use when writing the output
    #[arg(short = 'o', long, default_value = "{uuid}")]
    output_template: String,

    /// The seed to use when generating a v6 Guid (6 values)
    #[arg(
        short = '6',
        long,
        default_value = "1,2,3,4,5,6",
        value_parser = parse_v6_seed
    )]
    guid_v6_seed: UuidSeed,

    /// Print help
    #[arg(short = 'h', long, visible_short_alias = '?', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, visible_short_alias = '!', action = clap::ArgAction::Version)]
    version: Option<bool>,
}

struct FormatOptions {
    hyphenated: bool,
    uppercase: bool,
}

#[derive(Clone, Copy)]
struct GuidGenerateOptions {
    guid_version: GuidVersionType,
    v6_seed: UuidSeed,
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
    validate_output_template(&args.output_template).map_err(io::Error::other)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for sequence in 1..=args.count {
        let uuid_formatted = generate_uuid(args);
        let output = format_output(&args.output_template, &uuid_formatted, sequence);

        writeln!(handle, "{}", output)?;
    }

    Ok(())
}

/// Parse and validate the v6 node seed once, when the arguments are parsed.
fn parse_v6_seed(value: &str) -> Result<UuidSeed, String> {
    let parts: Vec<&str> = value.split(',').map(str::trim).collect();

    if parts.len() != 6 {
        return Err(format!(
            "expected 6 comma-separated values (0-255), got {}",
            parts.len()
        ));
    }

    let mut seed = [0u8; 6];
    for (seed_value, part) in seed.iter_mut().zip(parts) {
        *seed_value = part
            .parse::<u8>()
            .map_err(|_| format!("'{}' is not a valid byte value (expected 0-255)", part))?;
    }

    Ok(seed)
}

/// Check the output template once, before generating any values.
fn validate_output_template(output_format: &str) -> Result<(), String> {
    if output_format.is_empty() {
        return Ok(());
    }

    let mut vars = HashMap::new();
    vars.insert("sequence".to_string(), "0".to_string());
    vars.insert("uuid".to_string(), String::new());

    strfmt(output_format, &vars)
        .map(|_| ())
        .map_err(|e| format!("Invalid output template - {}", e))
}

fn generate_uuid(args: &Args) -> String {
    match args.uuid_type {
        UUIDType::Guid => {
            let format_options = FormatOptions {
                hyphenated: !args.non_hyphenated,
                uppercase: args.uppercase,
            };

            let generate_options = GuidGenerateOptions {
                guid_version: args.guid_version,
                v6_seed: args.guid_v6_seed,
            };

            let uuid = generate_guid(generate_options);
            format_guid(&uuid, &format_options)
        }
        UUIDType::NanoId => generate_nanoid(args.nanoid_length),
    }
}

fn generate_guid(options: GuidGenerateOptions) -> Uuid {
    match options.guid_version {
        GuidVersionType::V6 => Uuid::now_v6(&options.v6_seed),
        GuidVersionType::V7 => Uuid::now_v7(),
        GuidVersionType::V4 => Uuid::new_v4(),
    }
}

fn generate_nanoid(nanoid_length: usize) -> String {
    nanoid!(nanoid_length)
}

fn format_guid(uuid: &Uuid, format_options: &FormatOptions) -> String {
    let value: String = if format_options.hyphenated {
        uuid.hyphenated().to_string()
    } else {
        uuid.simple().to_string()
    };

    if format_options.uppercase {
        value.to_uppercase()
    } else {
        value.to_lowercase()
    }
}

fn format_output(output_format: &str, formatted_uuid: &str, sequence: u32) -> String {
    if output_format.is_empty() {
        return formatted_uuid.to_string();
    }

    let mut vars = HashMap::new();
    vars.insert("sequence".to_string(), sequence.to_string());
    vars.insert("uuid".to_string(), formatted_uuid.to_string());

    match strfmt(output_format, &vars) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: Invalid output template - {}", e);
            formatted_uuid.to_string()
        }
    }
}

#[cfg(test)]
mod main_tests;
