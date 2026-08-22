mod classifier;
mod planner;
mod scanner;

use clap::Parser;
use planner::ConflictResolution;
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
                let plan = planner::build_plan(directory.clone(), result);

                println!(
                    "{mode} mode selected for: {}",
                    plan.target_directory.display()
                );
                println!(
                    "{} directories would be created.",
                    plan.directories_to_create.len()
                );

                for directory in &plan.directories_to_create {
                    println!("   create: {}", directory.display());
                }

                println!("{} files would be moved.", plan.moves.len());

                for planned_move in &plan.moves {
                    match planned_move.conflict_resolution {
                        ConflictResolution::None => {
                            println!(
                                "    move: {} -> {}",
                                planned_move.source_path.display(),
                                planned_move.destination_path.display()
                            );
                        }
                        ConflictResolution::Renamed => {
                            println!(
                                "    move: {} -> {} (renamed to avoid conflict)",
                                planned_move.source_path.display(),
                                planned_move.destination_path.display()
                            );
                        }
                    }
                }

                println!("{} entries skipped.", plan.skipped.len());

                for skipped in &plan.skipped {
                    println!(
                        "    skipped: {} ({})",
                        skipped.name,
                        skipped.reason.as_message()
                    )
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
