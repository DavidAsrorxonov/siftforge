use clap::Parser;
use siftforge::executor;
use siftforge::history;
use siftforge::planner;
use siftforge::planner::ConflictResolution;
use siftforge::scanner;
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
                println!("{} files would be moved.", plan.moves.len());
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
                    println!();
                    println!("Applying organization plan...");

                    let directory_result = executor::create_plan_directories(&plan);
                    let move_result = executor::execute_plan_moves(&plan);

                    for directory in &directory_result.created_directories {
                        println!("  created: {}", directory.display());
                    }

                    for moved_file in &move_result.moved_files {
                        match moved_file.conflict_resolution {
                            ConflictResolution::None => {
                                println!(
                                    "  moved: {} -> {}",
                                    moved_file.source_path.display(),
                                    moved_file.destination_path.display()
                                );
                            }
                            ConflictResolution::Renamed => {
                                println!(
                                    "  moved: {} -> {} (renamed to avoid conflict)",
                                    moved_file.source_path.display(),
                                    moved_file.destination_path.display()
                                )
                            }
                        }
                    }

                    for failure in directory_result
                        .failures
                        .iter()
                        .chain(move_result.failures.iter())
                    {
                        match &failure.destination_path {
                            Some(destination_path) => {
                                eprintln!(
                                    "  failed: {} -> {} ({})",
                                    failure.source_path.display(),
                                    destination_path.display(),
                                    failure.reason
                                );
                            }
                            None => {
                                eprintln!(
                                    "  failed: {} ({})",
                                    failure.source_path.display(),
                                    failure.reason
                                );
                            }
                        }
                    }

                    let failure_count =
                        directory_result.failures.len() + move_result.failures.len();

                    let operation_record = history::build_operation_record(
                        plan.target_directory.clone(),
                        &directory_result,
                        &move_result,
                    );

                    match history::default_history_dir().and_then(|history_dir| {
                        history::write_operation_record_to_dir(&operation_record, &history_dir)
                    }) {
                        Ok(history_path) => {
                            println!("Saved operation history.");
                            println!("Operation ID: {}", operation_record.id);
                            println!("History file: {}", history_path.display());
                        }
                        Err(error) => {
                            eprintln!("Failed to save operation history: {error}");
                            std::process::exit(1);
                        }
                    }

                    if directory_result.has_failures() || move_result.has_failures() {
                        eprintln!("Apply completed with {failure_count} failure(s).");
                        std::process::exit(6)
                    }

                    println!("Completed successfully.")
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
