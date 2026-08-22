use std::fs;
use std::path::PathBuf;

use crate::planner::{ConflictResolution, OrganizationPlan, PlannedMove};

#[derive(Debug)]
pub struct ExecutionResult {
    pub created_directories: Vec<PathBuf>,
    pub moved_files: Vec<PlannedMove>,
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

#[cfg(test)]
mod tests {
    use super::create_plan_directories;
    use crate::planner::OrganizationPlan;

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
}
