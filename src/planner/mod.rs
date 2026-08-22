use std::path::PathBuf;

use crate::classifier;
use crate::classifier::Category;
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

pub fn build_plan(target_directory: PathBuf, scan_result: ScanResult) -> OrganizationPlan {
    let mut directories_to_create = Vec::new();
    let mut moves = Vec::new();

    for file in scan_result.files {
        let classification = classifier::classify_file_name(&file.name);
        let category_directory = target_directory.join(classification.category.folder_name());
        let destination_path = category_directory.join(&file.name);

        if !directories_to_create.contains(&category_directory) {
            directories_to_create.push(category_directory);
        }

        moves.push(PlannedMove {
            source_path: file.path,
            destination_path,
            category: classification.category,
            conflict_resolution: ConflictResolution::None,
        });
    }

    OrganizationPlan {
        target_directory,
        directories_to_create,
        moves,
        skipped: scan_result.skipped,
    }
}
