mod classifier;
mod executor;
mod planner;
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
                let plan = planner::build_plan(directory.clone(), result);

                println!("Scanning: {}", plan.target_directory.display());
                println!();
                println!("Proposed organization:");
                println!();

                for (category, count) in plan.category_counts() {
                    let file_word = if count == 1 { "file" } else { "files" };
                    println!("  {category:<12} {count:>3} {file_word}");
                }

                println!();
                println!(
                    "{} directories would be created.",
                    plan.directories_to_create.len()
                );
                println!("{} entries would be skipped.", plan.skipped.len());

                for skipped in &plan.skipped {
                    println!(
                        "  skipped: {} ({})",
                        skipped.name,
                        skipped.reason.as_message()
                    );
                }

                let conflict_count = plan.conflict_rename_count();

                if conflict_count == 1 {
                    println!("1 conflict would be renamed safely.");
                } else {
                    println!("{conflict_count} conflicts would be renamed safely.");
                }

                if !cli.apply {
                    println!();
                    println!(
                        "Run `siftforge {} --apply` to continue.",
                        plan.target_directory.display()
                    );
                } else {
                    eprintln!("Apply mode is not implemented yet.");
                    std::process::exit(1);
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
