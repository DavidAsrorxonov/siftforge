mod scanner;

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
        Some(directory) => match scanner::scan_directory(&directory) {
            Ok(result) => {
                let mode = if cli.apply { "Apply" } else { "Preview" };

                println!("{mode} mode selected for: {}", directory.display());
                println!("Found {} files.", result.files.len());

                for file in &result.files {
                    println!("  file: {} ({})", file.name, file.path.display());
                }

                println!("Skipped {} entries.", result.skipped.len());

                for skipped in &result.skipped {
                    println!(
                        "  skipped: {} ({})",
                        skipped.name,
                        skipped.reason.as_message()
                    );
                }
            }
            Err(error) => {
                eprintln!("Failed to scan {}: {error}", directory.display());
                std::process::exit(3);
            }
        },
        None => {
            println!("No directory provided. Try `siftforge --help`.");
        }
    }
}
