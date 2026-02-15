use clap::Parser;
use uuid::Uuid;
use nanoid::nanoid;
use strfmt::strfmt;
use std::collections::HashMap;

#[derive(clap::ValueEnum, Clone, Parser, Debug, PartialEq, Copy)]
#[clap(rename_all = "kebab-case")]
enum GuidVersionType {
    V4,
    V6,
    V7,
}

#[derive(clap::ValueEnum, Clone, Parser, Debug, PartialEq, Copy)]
#[clap(rename_all = "lowercase")]
enum UUIDType
{
    Guid,
    NanoId
}

/// Generate Unique IDs (UUIDs) with controlled output and formatting
#[derive(Parser, Debug)]
#[command(version, about, author, help_expected = true, after_help = "NOTE:\noutput-template supports: {uuid}, {sequence} dynamic values\n(See also : https://github.com/vitiral/strfmt)")]
struct Args {
    /// Number of times to generate
    #[arg(short = 'c', long, default_value_t = 1)]
    count: u8,

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
}

struct FormatOptions {
    hyphenated: bool,
    uppercase: bool,
}

fn main() {
    let args = Args::parse();

    for sequence in 1..=args.count
    {
        let uuid_formatted = generate_uuid(&args);
        let output = format_output(&args.output_template, &uuid_formatted, sequence);

        println!("{output}");
    }
}

fn generate_uuid(args: &Args) -> String
{
    if args.uuid_type == UUIDType::Guid
    {
        let format_options = FormatOptions
        {
            hyphenated: !args.non_hyphenated,
            uppercase: args.uppercase,
        };

        let uuid = generate_guid(args.guid_version);
        let uuid_formatted = format_guid(&uuid, &format_options);

        return uuid_formatted.clone();
    }
    else if args.uuid_type == UUIDType::NanoId
    {
        let uuid = generate_nanoid(args.nanoid_length);

        return uuid.clone();
    }

    String::new()
}

fn generate_guid(guid_version_type: GuidVersionType) -> Uuid
{
    if guid_version_type == GuidVersionType::V6 {
        let node = &[1, 2, 3, 4, 5, 6];

        return Uuid::now_v6(node);
    }
    if guid_version_type == GuidVersionType::V7 {
        return Uuid::now_v7();
    }

    Uuid::new_v4()
}

fn generate_nanoid(nanoid_length: usize) -> String
{
    let uuid = nanoid!(nanoid_length);

    uuid
}

fn format_guid(uuid: &Uuid, format_options: &FormatOptions) -> String
{
    let value: String = if format_options.hyphenated
    {
        uuid.hyphenated().to_string()
    }
    else
    {
        uuid.simple().to_string()
    };

    let value = if format_options.uppercase
    {
        value.to_uppercase()
    }
    else {
        value.to_lowercase()
    };

    value
}

fn format_output(output_format: &String, formatted_uuid: &String, sequence: u8) -> String {
    if output_format.is_empty() {
        return formatted_uuid.clone();
    }

    let mut vars = HashMap::new();
    vars.insert("sequence".to_string(), sequence.to_string());
    vars.insert("uuid".to_string(), formatted_uuid.to_string());

    let result  = strfmt(&output_format, &vars).unwrap();

    result
}
