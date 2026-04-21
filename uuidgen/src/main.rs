use clap::Parser;
use nanoid::nanoid;
use std::collections::HashMap;
use strfmt::strfmt;
use uuid::Uuid;

// Notes:
// - https://medium.com/@mattpatchava/std-fmt-56c412e30d74

#[derive(clap::ValueEnum, Clone, Parser, Debug, PartialEq, Copy)]
#[clap(rename_all = "kebab-case")]
enum GuidVersionType {
    V4,
    V6,
    V7,
}

#[derive(clap::ValueEnum, Clone, Parser, Debug, PartialEq, Copy)]
#[clap(rename_all = "lowercase")]
enum UUIDType {
    Guid,
    NanoId,
}

/// Generate Unique IDs (UUIDs) with controlled output and formatting
#[derive(Parser, Debug)]
#[command(
    version,
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
    #[arg(short = 'l', long, default_value = "21")]
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
    #[arg(short = '6', long, default_value = "1,2,3,4,5,6")]
    guid_v6_seed: String,

    /// Print help
    #[arg(short = 'h', long, visible_short_alias = '?', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, action = clap::ArgAction::Version)]
    version: Option<bool>,
}

struct FormatOptions {
    hyphenated: bool,
    uppercase: bool,
}

#[derive(Clone)]
struct GuidGenerateOptions {
    guid_version: GuidVersionType,
    v6_seed: String,
}

fn main() {
    let args = Args::parse();

    for sequence in 1..=args.count {
        let uuid_formatted = generate_uuid(&args);
        let output = format_output(&args.output_template, &uuid_formatted, sequence);

        println!("{output}");
    }
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
                v6_seed: args.guid_v6_seed.clone(),
            };

            let uuid = generate_guid(generate_options);
            format_guid(&uuid, &format_options)
        }
        UUIDType::NanoId => generate_nanoid(args.nanoid_length),
    }
}

fn generate_guid(options: GuidGenerateOptions) -> Uuid {
    if options.guid_version == GuidVersionType::V6 {
        let mut node = [1u8; 6];

        if !options.v6_seed.is_empty() {
            let mut seed_values: Vec<u8> = Vec::new();
            let mut invalid_values: Vec<&str> = Vec::new();

            for s in options.v6_seed.split(",") {
                match s.trim().parse::<u8>() {
                    Ok(value) => seed_values.push(value),
                    Err(_) => invalid_values.push(s),
                };
            }

            if seed_values.len() == 6 {
                for (i, &val) in seed_values.iter().enumerate() {
                    node[i] = val;
                }
            }

            if !invalid_values.is_empty() {
                eprintln!(
                    "Warning: unable to use seed values - {}",
                    invalid_values.join(", ")
                );
            }
        }

        return Uuid::now_v6(&node);
    }

    if options.guid_version == GuidVersionType::V7 {
        return Uuid::now_v7();
    }

    Uuid::new_v4()
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
