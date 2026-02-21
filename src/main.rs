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

    /// The seed to use when generating a v6 Guid (6 values)
    #[arg(short = '6', long, default_value = "1,2,3,4,5,6")]
    guid_v6_seed: String,
}

struct FormatOptions {
    hyphenated: bool,
    uppercase: bool,
}

struct GuidGenerateOptions {
    guid_version: GuidVersionType,
    v6_seed: String,
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

        let generate_options = GuidGenerateOptions
        {
            guid_version: args.guid_version,
            v6_seed: args.guid_v6_seed.clone(),
        };

        let uuid = generate_guid(generate_options);
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

fn parse_text_to_u8(str: String) -> u8 {
    let value = str.parse::<u8>()
        .expect("shit");

    value
}

fn generate_guid(options: GuidGenerateOptions) -> Uuid
{
    if options.guid_version == GuidVersionType::V6 {
        let node = &[1, 2, 3, 4, 5, 6];

        if !options.v6_seed.is_empty() {
            //let seed_values = options.v6_seed.split(",")
            //    .map(|str| str.trim().parse::<u8>().unwrap_or_default())
            //    .collect::<Vec<u8>>();

            let mut seed_values: Vec<u8> = Vec::new();
            let mut invalid_values: Vec<&str> = Vec::new();

            for str in options.v6_seed.split(",") {
                let parsed_value = match str.trim().parse::<u8>() {
                    Ok(value) => seed_values.push(value),
                    Err(err) => invalid_values.push(&str),
                };
            }

            if seed_values.len() == 6 {
                let _node: &[u8] = &seed_values;
            }
            if invalid_values.len() > 0 {
                println!("Warning: unable to use seed values - {}", invalid_values.join(", "));
            }
        }

        return Uuid::now_v6(node);
    }
    if options.guid_version == GuidVersionType::V7 {
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
