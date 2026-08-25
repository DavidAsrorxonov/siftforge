use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::classifier;
use crate::classifier::Category;
use crate::config::Config;
use crate::scanner::ScanResult;
use crate::scanner::SkippedEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictResolution {
    None,
    Renamed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedMove {
    pub source_path: PathBuf,
    pub destination_path: PathBuf,
    pub category: Category,
    pub conflict_resolution: ConflictResolution,
}

#[derive(Debug)]
pub struct OrganizationPlan {
    pub target_directory: PathBuf,
    pub directories_to_create: Vec<PathBuf>,
    pub moves: Vec<PlannedMove>,
    pub skipped: Vec<SkippedEntry>,
}

impl OrganizationPlan {
    pub fn category_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();

        for planned_move in &self.moves {
            let category_name = planned_move.category.folder_name();
            let count = counts.entry(category_name).or_insert(0);
            *count += 1;
        }

        counts
    }

    pub fn conflict_rename_count(&self) -> usize {
        self.moves
            .iter()
            .filter(|planned_move| planned_move.conflict_resolution == ConflictResolution::Renamed)
            .count()
    }
}

pub fn build_plan(
    target_directory: PathBuf,
    scan_result: ScanResult,
    config: &Config,
) -> OrganizationPlan {
    let mut directories_to_create = Vec::new();
    let mut moves = Vec::new();

    for file in scan_result.files {
        let classification = classifier::classify_file_name_with_config(&file.name, config);
        let category_directory = target_directory.join(classification.category.folder_name());
        let desired_destination_path = category_directory.join(&file.name);
        let (destination_path, conflict_resolution) =
            resolve_available_destination(desired_destination_path);

        if !directories_to_create.contains(&category_directory) {
            directories_to_create.push(category_directory);
        }

        moves.push(PlannedMove {
            source_path: file.path,
            destination_path,
            category: classification.category,
            conflict_resolution,
        });
    }

    OrganizationPlan {
        target_directory,
        directories_to_create,
        moves,
        skipped: scan_result.skipped,
    }
}

pub fn resolve_available_destination(destination_path: PathBuf) -> (PathBuf, ConflictResolution) {
    if !destination_path.exists() {
        return (destination_path, ConflictResolution::None);
    }

    let parent = destination_path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_default();
    let file_name = destination_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("file");

    let (stem, extension) = split_file_name_for_conflict(file_name);

    for counter in 1.. {
        let candidate_name = match extension {
            Some(extension) => format!("{stem} ({counter}).{extension}"),
            None => format!("{stem} ({counter})"),
        };

        let candidate_path = parent.join(candidate_name);

        if !candidate_path.exists() {
            return (candidate_path, ConflictResolution::Renamed);
        }
    }

    unreachable!("conflict resolution loop should always return");
}

fn split_file_name_for_conflict(file_name: &str) -> (&str, Option<&str>) {
    for compound_extension in ["tar.gz", "tar.bz2", "tar.xz", "tar.zst"] {
        let suffix = format!(".{compound_extension}");

        if let Some(stem) = file_name.strip_suffix(&suffix) {
            return (stem, Some(compound_extension));
        }
    }

    match file_name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() && !extension.is_empty() => {
            (stem, Some(extension))
        }
        _ => (file_name, None),
    }
}

#[cfg(test)]
mod tests {
    use super::{resolve_available_destination, ConflictResolution};
    use super::{OrganizationPlan, PlannedMove};
    use crate::classifier::Category;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn keeps_destination_when_no_conflict_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let destination = temp_dir.path().join("report.pdf");

        let (resolved, conflict_resolution) = resolve_available_destination(destination.clone());

        assert_eq!(resolved, destination);
        assert_eq!(conflict_resolution, ConflictResolution::None);
    }

    #[test]
    fn renames_destination_when_conflict_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let destination = temp_dir.path().join("report.pdf");

        fs::write(&destination, "existing").unwrap();

        let (resolved, conflict_resolution) = resolve_available_destination(destination);

        assert_eq!(resolved.file_name().unwrap(), "report (1).pdf");
        assert_eq!(conflict_resolution, ConflictResolution::Renamed);
    }

    #[test]
    fn increments_until_available_name_is_found() {
        let temp_dir = tempfile::tempdir().unwrap();

        fs::write(temp_dir.path().join("report.pdf"), "existing").unwrap();
        fs::write(temp_dir.path().join("report (1).pdf"), "existing").unwrap();

        let destination = temp_dir.path().join("report.pdf");
        let (resolved, conflict_resolution) = resolve_available_destination(destination);

        assert_eq!(resolved.file_name().unwrap(), "report (2).pdf");
        assert_eq!(conflict_resolution, ConflictResolution::Renamed);
    }

    #[test]
    fn preserves_compound_extension_when_renaming() {
        let temp_dir = tempfile::tempdir().unwrap();
        let destination = temp_dir.path().join("backup.tar.gz");

        fs::write(&destination, "existing").unwrap();

        let (resolved, conflict_resolution) = resolve_available_destination(destination);

        assert_eq!(resolved.file_name().unwrap(), "backup (1).tar.gz");
        assert_eq!(conflict_resolution, ConflictResolution::Renamed);
    }

    #[test]
    fn counts_moves_by_category() {
        let plan = OrganizationPlan {
            target_directory: PathBuf::from("."),
            directories_to_create: Vec::new(),
            moves: vec![
                PlannedMove {
                    source_path: PathBuf::from("photo.png"),
                    destination_path: PathBuf::from("Images/photo.png"),
                    category: Category::Images,
                    conflict_resolution: ConflictResolution::None,
                },
                PlannedMove {
                    source_path: PathBuf::from("notes.md"),
                    destination_path: PathBuf::from("Documents/notes.md"),
                    category: Category::Documents,
                    conflict_resolution: ConflictResolution::None,
                },
                PlannedMove {
                    source_path: PathBuf::from("report.pdf"),
                    destination_path: PathBuf::from("Documents/report.pdf"),
                    category: Category::Documents,
                    conflict_resolution: ConflictResolution::Renamed,
                },
            ],
            skipped: Vec::new(),
        };

        let counts = plan.category_counts();

        assert_eq!(counts.get("Images"), Some(&1));
        assert_eq!(counts.get("Documents"), Some(&2));
    }

    #[test]
    fn counts_conflict_renames() {
        let plan = OrganizationPlan {
            target_directory: PathBuf::from("."),
            directories_to_create: Vec::new(),
            moves: vec![
                PlannedMove {
                    source_path: PathBuf::from("notes.md"),
                    destination_path: PathBuf::from("Documents/notes.md"),
                    category: Category::Documents,
                    conflict_resolution: ConflictResolution::None,
                },
                PlannedMove {
                    source_path: PathBuf::from("report.pdf"),
                    destination_path: PathBuf::from("Documents/report (1).pdf"),
                    category: Category::Documents,
                    conflict_resolution: ConflictResolution::Renamed,
                },
            ],
            skipped: Vec::new(),
        };

        assert_eq!(plan.conflict_rename_count(), 1);
    }
}
