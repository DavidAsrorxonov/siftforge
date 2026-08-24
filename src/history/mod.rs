use crate::executor::{ExecutionFailure, ExecutionResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub version: u32,
    pub id: String,
    pub target_directory: PathBuf,
    pub created_at: String,
    pub completed_at: String,
    pub created_directories: Vec<PathBuf>,
    pub moves: Vec<HistoryMove>,
    pub failures: Vec<HistoryFailure>,
    pub status: OperationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryMove {
    pub source_path: PathBuf,
    pub destination_path: PathBuf,
    pub conflict_resolution: String,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFailure {
    pub source_path: PathBuf,
    pub destination_path: Option<PathBuf>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationStatus {
    Completed,
    CompletedWithErrors,
}

pub fn build_operation_record(
    target_directory: PathBuf,
    directory_result: &ExecutionResult,
    move_result: &ExecutionResult,
) -> OperationRecord {
    let created_at = current_timestamp();
    let completed_at = created_at.clone();

    let failures = directory_result
        .failures
        .iter()
        .chain(move_result.failures.iter())
        .map(history_failure_from_execution_failure)
        .collect::<Vec<_>>();

    let moves = move_result
        .moved_files
        .iter()
        .map(|moved_file| HistoryMove {
            source_path: moved_file.source_path.clone(),
            destination_path: moved_file.destination_path.clone(),
            conflict_resolution: format!("{:?}", moved_file.conflict_resolution),
            completed_at: completed_at.clone(),
        })
        .collect::<Vec<_>>();

    let status = if failures.is_empty() {
        OperationStatus::Completed
    } else {
        OperationStatus::CompletedWithErrors
    };

    OperationRecord {
        version: 1,
        id: created_at.clone(),
        target_directory,
        created_at,
        completed_at,
        created_directories: directory_result.created_directories.clone(),
        moves,
        failures,
        status,
    }
}

fn history_failure_from_execution_failure(failure: &ExecutionFailure) -> HistoryFailure {
    HistoryFailure {
        source_path: failure.source_path.clone(),
        destination_path: failure.destination_path.clone(),
        reason: failure.reason.clone(),
    }
}

fn current_timestamp() -> String {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();

    format!("operation-{}", duration.as_millis())
}

#[cfg(test)]
mod tests {
    use super::{build_operation_record, OperationStatus};
    use crate::executor::{ExecutedMove, ExecutionFailure, ExecutionResult};
    use crate::planner::ConflictResolution;
    use std::path::PathBuf;

    #[test]
    fn builds_completed_operation_record() {
        let mut directory_result = ExecutionResult::new();
        directory_result
            .created_directories
            .push(PathBuf::from("Images"));

        let mut move_result = ExecutionResult::new();
        move_result.moved_files.push(ExecutedMove {
            source_path: PathBuf::from("photo.png"),
            destination_path: PathBuf::from("Images/photo.png"),
            conflict_resolution: ConflictResolution::None,
        });

        let record = build_operation_record(PathBuf::from("."), &directory_result, &move_result);

        assert_eq!(record.version, 1);
        assert_eq!(record.target_directory, PathBuf::from("."));
        assert_eq!(record.created_directories, vec![PathBuf::from("Images")]);
        assert_eq!(record.moves.len(), 1);
        assert!(record.failures.is_empty());
        assert_eq!(record.status, OperationStatus::Completed);
    }

    #[test]
    fn builds_completed_with_errors_operation_record() {
        let directory_result = ExecutionResult::new();

        let mut move_result = ExecutionResult::new();
        move_result.failures.push(ExecutionFailure {
            source_path: PathBuf::from("missing.pdf"),
            destination_path: Some(PathBuf::from("Documents/missing.pdf")),
            reason: "source file does not exist".to_string(),
        });

        let record = build_operation_record(PathBuf::from("."), &directory_result, &move_result);

        assert_eq!(record.moves.len(), 0);
        assert_eq!(record.failures.len(), 1);
        assert_eq!(record.status, OperationStatus::CompletedWithErrors);
    }
}
