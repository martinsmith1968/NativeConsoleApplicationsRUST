use clap::Parser;
use std::fs;
use std::io;

mod hashers;
use hashers::hash_content;

/// Generate a hash of text or file contents
#[derive(Parser, Debug)]
#[command(
    version,
    about = concat!("Generate a hash of text or file contents\nCopyright \u{00A9} 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    author,
    help_expected = true,
    disable_help_flag = true,
    disable_version_flag = true,
    after_help = "Examples:\n  hashcalc --text \"Hello World\"\n  hashcalc --text \"Hello World\" --algorithm md5\n  hashcalc --file myfile.txt\n  hashcalc --file myfile.txt --algorithm sha512\n  hashcalc --file myfile.txt --write"
)]
struct Args {
    /// The text to generate a hash for (mutually exclusive with --file)
    #[arg(short, long)]
    text: Option<String>,

    /// Path to file to hash
    #[arg(short, long)]
    file: Option<String>,

    /// Write output to file instead of stdout (requires --file)
    #[arg(short = 'w', long)]
    write: bool,

    /// Hash algorithm to use: sha1, md5, sha256, sha512, base64
    #[arg(short, long, default_value = "sha256")]
    algorithm: String,

    /// Print help
    #[arg(short = 'h', long, visible_short_alias = '?', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn read_file_contents(path: &str) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => format!("File not found: {}", path),
        io::ErrorKind::PermissionDenied => format!("Permission denied: {}", path),
        io::ErrorKind::InvalidData => format!("Invalid file data: {}", path),
        _ => format!("Failed to read file '{}': {}", path, e),
    })
}

fn main() {
    let args = Args::parse();

    let content_bytes = match (&args.text, &args.file) {
        (Some(text), None) => text.as_bytes().to_vec(),
        (None, Some(file_path)) => match read_file_contents(file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        (Some(_), Some(_)) => {
            eprintln!("Error: specify either text or --file, not both");
            std::process::exit(1);
        }
        (None, None) => {
            eprintln!("Error: provide either text or --file option");
            std::process::exit(1);
        }
    };

    match hash_content(&content_bytes, &args.algorithm) {
        Ok(hash_output) => {
            // Validate write flag usage
            if args.write && args.file.is_none() {
                eprintln!("Error: --write requires --file");
                std::process::exit(1);
            }
            if args.write && args.text.is_some() {
                eprintln!("Error: --write cannot be used with --text");
                std::process::exit(1);
            }

            let identifier = match (&args.text, &args.file) {
                (Some(text), None) => text.clone(),
                (None, Some(file_path)) => std::path::Path::new(file_path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(file_path)
                    .to_string(),
                _ => String::new(),
            };

            let output_line = format!(
                "{} [{}] : {}",
                identifier,
                args.algorithm.to_lowercase(),
                hash_output
            );

            if args.write {
                // Construct output path in same directory with {filename}.{algorithm}
                let input_path = std::path::Path::new(args.file.as_ref().unwrap());
                let dir = input_path
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."));
                let file_name = input_path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output");
                let alg = args.algorithm.to_lowercase();
                let out_file_name = format!("{}.{}", file_name, alg);
                let out_path = dir.join(out_file_name);

                if let Err(e) = fs::write(&out_path, output_line.as_bytes()) {
                    eprintln!("Error writing to file '{}': {}", out_path.display(), e);
                    std::process::exit(1);
                }
                // Success: exit quietly
                return;
            }

            println!("{}", output_line);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}


#[cfg(test)]
mod main_tests;
