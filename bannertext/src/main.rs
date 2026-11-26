use bannertext::{Args, run};
use clap::Parser;
use std::io;
use std::process::ExitCode;

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
