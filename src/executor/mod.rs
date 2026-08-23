use std::fs;
use std::path::PathBuf;

use crate::planner::{ConflictResolution, OrganizationPlan, PlannedMove};

#[derive(Debug)]
pub struct ExecutionResult {
    pub created_directories: Vec<PathBuf>,
    pub moved_files: Vec<ExecutedMove>,
    pub failures: Vec<ExecutionFailure>,
}

#[derive(Debug)]
pub struct ExecutedMove {
    pub source_path: PathBuf,
    pub destination_path: PathBuf,
    pub conflict_resolution: ConflictResolution,
}

#[derive(Debug)]
pub struct ExecutionFailure {
    pub source_path: PathBuf,
    pub destination_path: Option<PathBuf>,
    pub reason: String,
}

impl ExecutionResult {
    pub fn new() -> Self {
        Self {
            created_directories: Vec::new(),
            moved_files: Vec::new(),
            failures: Vec::new(),
        }
    }

    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
    }
}

impl From<&PlannedMove> for ExecutedMove {
    fn from(planned_move: &PlannedMove) -> Self {
        Self {
            source_path: planned_move.source_path.clone(),
            destination_path: planned_move.destination_path.clone(),
            conflict_resolution: planned_move.conflict_resolution.clone(),
        }
    }
}

pub fn create_plan_directories(plan: &OrganizationPlan) -> ExecutionResult {
    let mut result = ExecutionResult::new();

    for directory in &plan.directories_to_create {
        if directory.exists() {
            continue;
        }

        match fs::create_dir(directory) {
            Ok(()) => {
                result.created_directories.push(directory.clone());
            }
            Err(error) => {
                result.failures.push(ExecutionFailure {
                    source_path: plan.target_directory.clone(),
                    destination_path: Some(directory.clone()),
                    reason: format!("failed to create directory: {error}"),
                });
            }
        }
    }

    result
}

pub fn execute_plan_moves(plan: &OrganizationPlan) -> ExecutionResult {
    let mut result = ExecutionResult::new();

    for planned_move in &plan.moves {
        if !planned_move.source_path.exists() {
            result.failures.push(ExecutionFailure {
                source_path: planned_move.source_path.clone(),
                destination_path: Some(planned_move.destination_path.clone()),
                reason: "source file does not exist".to_string(),
            });
            continue;
        }

        if !planned_move.source_path.is_file() {
            result.failures.push(ExecutionFailure {
                source_path: planned_move.source_path.clone(),
                destination_path: Some(planned_move.destination_path.clone()),
                reason: "source path is not a file".to_string(),
            });
            continue;
        }

        if planned_move.destination_path.exists() {
            result.failures.push(ExecutionFailure {
                source_path: planned_move.source_path.clone(),
                destination_path: Some(planned_move.destination_path.clone()),
                reason: "destination already exists".to_string(),
            });
            continue;
        }

        if let Some(parent) = planned_move.destination_path.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                result.failures.push(ExecutionFailure {
                    source_path: planned_move.source_path.clone(),
                    destination_path: Some(planned_move.destination_path.clone()),
                    reason: format!("failed to create destination directory: {error}"),
                });
                continue;
            }
        }

        match fs::rename(&planned_move.source_path, &planned_move.destination_path) {
            Ok(()) => {
                result.moved_files.push(ExecutedMove::from(planned_move));
            }
            Err(error) => {
                result.failures.push(ExecutionFailure {
                    source_path: planned_move.source_path.clone(),
                    destination_path: Some(planned_move.destination_path.clone()),
                    reason: format!("failed to move file: {error}"),
                });
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::create_plan_directories;
    use super::execute_plan_moves;
    use crate::classifier::Category;
    use crate::planner::{ConflictResolution, OrganizationPlan, PlannedMove};
    use std::fs;

    #[test]
    fn creates_missing_plan_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let images = temp_dir.path().join("Images");
        let documents = temp_dir.path().join("Documents");

        let plan = OrganizationPlan {
            target_directory: temp_dir.path().to_path_buf(),
            directories_to_create: vec![images.clone(), documents.clone()],
            moves: Vec::new(),
            skipped: Vec::new(),
        };

        let result = create_plan_directories(&plan);

        assert!(images.is_dir());
        assert!(documents.is_dir());
        assert_eq!(result.created_directories.len(), 2);
        assert!(result.failures.is_empty());
    }

    #[test]
    fn ignores_directories_that_already_exist() {
        let temp_dir = tempfile::tempdir().unwrap();
        let images = temp_dir.path().join("Images");

        std::fs::create_dir(&images).unwrap();

        let plan = OrganizationPlan {
            target_directory: temp_dir.path().to_path_buf(),
            directories_to_create: vec![images.clone()],
            moves: Vec::new(),
            skipped: Vec::new(),
        };

        let result = create_plan_directories(&plan);

        assert!(images.is_dir());
        assert!(result.created_directories.is_empty());
        assert!(result.failures.is_empty());
    }

    #[test]
    fn moves_planned_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("photo.png");
        let destination = temp_dir.path().join("Images").join("photo.png");

        fs::write(&source, "image").unwrap();

        let plan = OrganizationPlan {
            target_directory: temp_dir.path().to_path_buf(),
            directories_to_create: Vec::new(),
            moves: vec![PlannedMove {
                source_path: source.clone(),
                destination_path: destination.clone(),
                category: Category::Images,
                conflict_resolution: ConflictResolution::None,
            }],
            skipped: Vec::new(),
        };

        let result = execute_plan_moves(&plan);

        assert!(!source.exists());
        assert_eq!(fs::read_to_string(&destination).unwrap(), "image");
        assert_eq!(result.moved_files.len(), 1);
        assert!(result.failures.is_empty());
    }

    #[test]
    fn does_not_overwrite_existing_destination() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("report.pdf");
        let destination_directory = temp_dir.path().join("Documents");
        let destination = destination_directory.join("report.pdf");

        fs::create_dir(&destination_directory).unwrap();
        fs::write(&source, "new").unwrap();
        fs::write(&destination, "existing").unwrap();

        let plan = OrganizationPlan {
            target_directory: temp_dir.path().to_path_buf(),
            directories_to_create: Vec::new(),
            moves: vec![PlannedMove {
                source_path: source.clone(),
                destination_path: destination.clone(),
                category: Category::Documents,
                conflict_resolution: ConflictResolution::None,
            }],
            skipped: Vec::new(),
        };

        let result = execute_plan_moves(&plan);

        assert_eq!(fs::read_to_string(&source).unwrap(), "new");
        assert_eq!(fs::read_to_string(&destination).unwrap(), "existing");
        assert!(result.moved_files.is_empty());
        assert_eq!(result.failures.len(), 1);
    }

    #[test]
    fn records_failure_when_source_is_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("missing.pdf");
        let destination = temp_dir.path().join("Documents").join("missing.pdf");

        let plan = OrganizationPlan {
            target_directory: temp_dir.path().to_path_buf(),
            directories_to_create: Vec::new(),
            moves: vec![PlannedMove {
                source_path: source,
                destination_path: destination,
                category: Category::Documents,
                conflict_resolution: ConflictResolution::None,
            }],
            skipped: Vec::new(),
        };

        let result = execute_plan_moves(&plan);

        assert!(result.moved_files.is_empty());
        assert_eq!(result.failures.len(), 1);
    }
}
