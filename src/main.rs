use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "siftforge")]
#[command(version)]
#[command(about = "A safe, cross-platform CLI for organizing cluttered directories.")]
struct Cli {
    /// Directory to organize.
    directory: Option<PathBuf>,

    /// Apply the planned organization.
    #[arg(long)]
    apply: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.directory {
        Some(directory) => {
            if cli.apply {
                println!("Apply mode selected for: {}", directory.display());
            } else {
                println!("Preview mode selected for: {}", directory.display());
            }
        }
        None => {
            println!("No directory provided. Try `siftforge --help`.");
        }
    }
}
