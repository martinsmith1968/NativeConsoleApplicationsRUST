use clap::Parser;
use uuid::Uuid;

#[derive(clap::ValueEnum, Clone, Parser, Debug, PartialEq, Copy)]
#[clap(rename_all = "kebab-case")]
enum GuidVersionType {
    V4,
    V6,
    V7,
}

/// Generate GUID(s) with controlled output and formatting
#[derive(Parser, Debug)]
#[command(version, about, after_help = "NOTE: \noutput-format supports: {uuid}, {sequence} dynamic values")]
struct Args {
    /// Number of times to generate
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// The version of GUID to generate
    #[arg(short = 'v', long, default_value = "v4")]
    guid_version: GuidVersionType,

    /// Format the GUID without Hyphens
    #[arg(short = 'y', long, default_value_t = false)]
    non_hyphenated: bool,

    /// Covert the GUID to Upper case values
    #[arg(short = 'u', long, default_value_t = false)]
    uppercase: bool,

    // The format to use when writing the output
    #[arg(short = 'o', long, default_value = "{uuid}")]
    output_format: String
}

struct FormatOptions {
    hyphenated: bool,
    uppercase: bool,
}

fn main() {
    let args = Args::parse();

    let guid_version = args.guid_version;
    let format_options = FormatOptions
    {
        hyphenated: !args.non_hyphenated,
        uppercase: args.uppercase,
    };
    let output_format = args.output_format;

    for sequence in 1..=args.count {
        let uuid = generate_guid(guid_version);

        let uuid_formatted = format_guid(uuid, &format_options);

        let output = format_output(&output_format, &uuid_formatted.clone(), sequence);

        println!("{}", output);
    }
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

fn format_guid(uuid: Uuid, format_options: &FormatOptions) -> String
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

    return value;
}

fn format_output(output_format: &String, formatted_uuid: &String, sequence: u8) -> String {
    if output_format.is_empty() {
        return formatted_uuid.clone();
    }

    let sequence_str = sequence.to_string();

    let result  = output_format;

    let result = result.replace("{sequence}", &sequence_str);
    let result = result.replace("{uuid}", &formatted_uuid);

    return result;
}
