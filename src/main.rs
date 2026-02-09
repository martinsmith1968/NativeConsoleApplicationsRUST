use clap::Parser;
use uuid::Uuid;

#[derive(clap::ValueEnum, Clone, Parser, Debug, PartialEq, Copy)]
#[clap(rename_all = "kebab-case")]
enum GuidVersionType {
    V4,
    V6,
    V7,
}

#[derive(clap::ValueEnum, Clone, Parser, Debug, PartialEq, Copy)]
#[clap(rename_all = "kebab-case")]
enum GuidFormatType {
    Hyphenated,
    UppercaseHyphenated,
    Digits,
    UppercaseDigits,
}

/// Generate GUID(s)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to generate
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// The version of GUID to generate
    #[arg(short = 'v', long, default_value = "v4")]
    guid_version: GuidVersionType,

    // How to format the generated Guid
    #[arg(short = 'f', long, default_value = "hyphenated")]
    guid_format: GuidFormatType,

    // The format to use when writing the output
    #[arg(short = 'o', long, default_value = "{uuid}")]
    output_format: String
}

fn main() {
    let args = Args::parse();

    let guid_version = args.guid_version;
    let guid_format = args.guid_format;
    let output_format = args.output_format;

    for sequence in 1..=args.count {
        let uuid = generate_guid(guid_version);

        let uuid_formatted = format_guid(uuid, guid_format);

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

fn format_guid(uuid: Uuid, format: GuidFormatType) -> String
{
    if format == GuidFormatType::Hyphenated {
        return uuid.hyphenated().to_string().to_lowercase();
    }

    if format == GuidFormatType::UppercaseHyphenated {
        return uuid.hyphenated().to_string().to_uppercase();
    }

    if format == GuidFormatType::Digits {
        return uuid.simple().to_string().to_lowercase();
    }

    if format == GuidFormatType::UppercaseDigits {
        return uuid.simple().to_string().to_uppercase();
    }

    return uuid.to_string();
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
