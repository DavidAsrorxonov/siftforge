use clap::{Parser, Subcommand};
use siftforge::executor;
use siftforge::history;
use siftforge::planner;
use siftforge::planner::ConflictResolution;
use siftforge::scanner;
use siftforge::undo;
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

    /// Path to a SiftForge config file.
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show recent SiftForge operations.
    History,

    /// Undo the latest SiftForge operation.
    Undo,

    /// Create a starter siftforge.yml config file.
    Init,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Command::History => {
                match history::default_history_dir()
                    .and_then(|history_dir| history::read_operation_records_from_dir(&history_dir))
                {
                    Ok(records) => {
                        println!("Recent SiftForge operations:");
                        println!();

                        if records.is_empty() {
                            println!("No operations found.");
                        }

                        for (index, record) in records.iter().enumerate() {
                            let moved_count = record.moves.len();
                            let moved_word = "moved";

                            println!(
                                "{}. {}   {}   {}   {}",
                                index + 1,
                                record.id,
                                record.target_directory.display(),
                                moved_count,
                                moved_word
                            );
                        }
                    }
                    Err(error) => {
                        eprintln!("Failed to read history: {error}");
                        std::process::exit(1)
                    }
                }

                return;
            }
            Command::Undo => {
                match history::default_history_dir().and_then(|history_dir| {
                    let record_path =
                        history::latest_undoable_operation_record_path_from_dir(&history_dir)?
                            .ok_or_else(|| {
                                std::io::Error::new(
                                    std::io::ErrorKind::NotFound,
                                    "no undoable operation history found",
                                )
                            })?;

                    let record = history::read_operation_record(&record_path)?;

                    Ok((record, record_path))
                }) {
                    Ok((mut record, record_path)) => {
                        println!("Undoing operation: {}", record.id);
                        println!();

                        let undo_result = undo::undo_operation(&record);

                        history::mark_operation_undone(
                            &mut record,
                            undo_result.restored_files.len(),
                            undo_result.skipped_files.len(),
                            undo_result.has_warnings(),
                        );

                        if let Err(error) =
                            history::write_operation_record_to_path(&record, &record_path)
                        {
                            eprintln!("Failed to update operation history: {error}");
                            std::process::exit(1);
                        }

                        for restored_file in &undo_result.restored_files {
                            println!(
                                "  restored: {} -> {}",
                                restored_file.from_path.display(),
                                restored_file.to_path.display()
                            );
                        }

                        for skipped_file in &undo_result.skipped_files {
                            eprintln!(
                                "  skipped: {} -> {} ({})",
                                skipped_file.from_path.display(),
                                skipped_file.to_path.display(),
                                skipped_file.reason
                            );
                        }

                        for directory in &undo_result.removed_directories {
                            println!("  removed directory: {}", directory.display());
                        }

                        for failure in &undo_result.directory_failures {
                            eprintln!(
                                "  directory warning: {} ({})",
                                failure.directory_path.display(),
                                failure.reason
                            );
                        }

                        println!();
                        println!("{} files restored.", undo_result.restored_files.len());
                        println!("{} files skipped.", undo_result.skipped_files.len());
                        println!(
                            "{} directories removed.",
                            undo_result.removed_directories.len()
                        );

                        if undo_result.has_warnings() {
                            eprintln!("Undo completed with warnings.");
                            std::process::exit(7);
                        }

                        println!("Undo completed successfully.");
                    }
                    Err(error) => {
                        eprintln!("Failed to undo latest operation: {error}");
                        std::process::exit(1);
                    }
                }

                return;
            }

            Command::Init => {
                let config_path = std::env::current_dir()
                    .map(|directory| directory.join("siftforge.yml"))
                    .unwrap_or_else(|_| PathBuf::from("siftforge.yml"));

                match siftforge::config::write_starter_config(&config_path) {
                    Ok(()) => {
                        println!("Created {}", config_path.display());
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                        eprintln!("Config file already exists: {}", config_path.display());
                        std::process::exit(1);
                    }
                    Err(error) => {
                        eprintln!("Failed to create config file: {error}");
                        std::process::exit(1);
                    }
                }

                return;
            }
        }
    }

    match cli.directory {
        Some(directory) => {
            let config =
                match siftforge::config::load_effective_config(&directory, cli.config.as_deref()) {
                    Ok(config) => config,
                    Err(error) => {
                        eprintln!("Failed to load config: {error}");
                        std::process::exit(4);
                    }
                };

            if !config.categories.is_empty() {
                println!(
                    "Loaded {} custom config categories.",
                    config.categories.len()
                );
            }

            match scanner::scan_directory(&directory) {
                Ok(result) => {
                    let plan = planner::build_plan(directory.clone(), result, &config);

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
            }
        }
        None => {
            println!("No directory provided. Try `siftforge --help`.");
        }
    }
}
