use clap::Parser;
use sha2::{Sha256, Digest};
use std::fs;
use std::io;

// Notes:
// - https://mojoauth.com/hashing/sha-256-in-rust#validating-and-testing-sha-256-in-rust
// - https://ssojet.com/hashing/sha-256-in-rust

//#[derive(clap::ValueEnum, Clone, Parser, Debug, PartialEq, Copy)]
//#[clap(rename_all = "kebab-case")]
//enum HashType {
//    sha1,
//    md5,
//    sha256,
//    sha512,
//    base64,
//}

/// Generate a SHA256 hash of text or file contents
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    author,
    help_expected = true,
)]
struct Args {
    // The version of hash to generate
    //#[arg(short = 'v', long, default_value = "sha256")]
    //hash_type: HashType,

    /// The text to generate a hash for (mutually exclusive with --file)
    text: Option<String>,

    /// Path to file to hash
    #[arg(short, long)]
    file: Option<String>,
}

fn read_file_contents(path: &str) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => format!("File not found: {}", path),
            io::ErrorKind::PermissionDenied => format!("Permission denied: {}", path),
            io::ErrorKind::InvalidData => format!("Invalid file data: {}", path),
            _ => format!("Failed to read file '{}': {}", path, e),
        }
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
        },
        (None, None) => {
            eprintln!("Error: provide either text or --file option");
            std::process::exit(1);
        },
    };

    let mut hasher = Sha256::new();
    hasher.update(&content_bytes);

    let result = hasher.finalize();
    let bytes = result.to_vec();
    let hex_string = hex::encode(&bytes);

    println!("{:02x?}", result);
    println!("{}", hex_string);
}
