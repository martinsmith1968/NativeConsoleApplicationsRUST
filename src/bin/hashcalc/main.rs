use clap::Parser;
use sha2::{Sha256, Digest};

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

/// Generate a hash of a file / text
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

    /// The text to generate a hash for
    text: String
}

fn main() {
    let args = Args::parse();

    let mut hasher = Sha256::new();

    hasher.update(args.text.as_bytes());

    let result = hasher.finalize();
    let bytes = result.to_vec();

    let hex_string = hex::encode(&bytes);
    
    println!("{:02x?}", result);
    println!("{}", hex_string);
}
