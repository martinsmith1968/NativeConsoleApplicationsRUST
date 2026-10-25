use clap::{ArgGroup, Parser};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

mod hashers;
use hashers::{Algorithm, hash_content};

/// Generate a hash of text or file contents
#[derive(Parser, Debug)]
#[command(
    version,
    bin_name = env!("CARGO_PKG_NAME"),
    about = concat!("hashcalc v", env!("CARGO_PKG_VERSION"), " - Generate a hash of text or file contents\nCopyright \u{00A9} 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    long_about = concat!("hashcalc v", env!("CARGO_PKG_VERSION"), " - Generate a hash of text or file contents\nCopyright \u{00A9} 2025-", env!("BUILD_YEAR"), " Martin Smith"),
    author,
    help_expected = true,
    disable_help_flag = true,
    disable_version_flag = true,
    group = ArgGroup::new("input").required(true).args(["text", "file"]),
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
    #[arg(short = 'w', long, conflicts_with = "text")]
    write: bool,

    /// Hash algorithm to use
    #[arg(
        short,
        long,
        value_enum,
        ignore_case = true,
        default_value_t = Algorithm::Sha256
    )]
    algorithm: Algorithm,

    /// Print help
    #[arg(short = 'h', long, visible_short_alias = '?', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, visible_short_alias = '!', action = clap::ArgAction::Version)]
    version: Option<bool>,
}

/// Turn a file I/O error into a message that names the file and keeps the error kind.
fn describe_file_error(path: &str, error: io::Error) -> io::Error {
    let message = match error.kind() {
        io::ErrorKind::NotFound => format!("File not found: {}", path),
        io::ErrorKind::PermissionDenied => format!("Permission denied: {}", path),
        io::ErrorKind::InvalidData => format!("Invalid file data: {}", path),
        _ => format!("Failed to read file '{}': {}", path, error),
    };
    io::Error::new(error.kind(), message)
}

/// Stream a file through the selected algorithm instead of reading it into memory.
fn hash_file(path: &str, algorithm: Algorithm) -> io::Result<String> {
    let mut file = File::open(path).map_err(|error| describe_file_error(path, error))?;
    algorithm
        .hash_reader(&mut file)
        .map_err(|error| describe_file_error(path, error))
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
    let (identifier, hash_output) = match (&args.text, &args.file) {
        (Some(text), _) => (text.as_str(), hash_content(text.as_bytes(), args.algorithm)),
        (None, Some(file_path)) => {
            let identifier = Path::new(file_path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(file_path);
            (identifier, hash_file(file_path, args.algorithm)?)
        }
        (None, None) => {
            return Err(io::Error::other("provide either --text or --file"));
        }
    };

    let output_line = format!("{} [{}] : {}", identifier, args.algorithm, hash_output);

    if args.write {
        let input_path = args
            .file
            .as_deref()
            .ok_or_else(|| io::Error::other("--write requires --file"))?;
        write_output_file(Path::new(input_path), args.algorithm, &output_line)?;
        // Success: exit quietly
        return Ok(());
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", output_line)
}

fn write_output_file(input_path: &Path, algorithm: Algorithm, output_line: &str) -> io::Result<()> {
    // Construct output path in same directory with {filename}.{algorithm}
    let dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = input_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let out_path = dir.join(format!("{}.{}", file_name, algorithm));

    fs::write(&out_path, output_line)
        .map_err(|e| io::Error::other(format!("failed to write '{}': {}", out_path.display(), e)))
}

#[cfg(test)]
mod main_tests;
