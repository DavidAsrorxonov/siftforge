use std::fs;
use std::path::PathBuf;

use crate::history::OperationRecord;

#[derive(Debug)]
pub struct UndoResult {
    pub restored_files: Vec<RestoredFile>,
    pub skipped_files: Vec<UndoSkippedFile>,
    pub removed_directories: Vec<PathBuf>,
    pub directory_failures: Vec<UndoDirectoryFailure>,
}

#[derive(Debug)]
pub struct RestoredFile {
    pub from_path: PathBuf,
    pub to_path: PathBuf,
}

#[derive(Debug)]
pub struct UndoSkippedFile {
    pub from_path: PathBuf,
    pub to_path: PathBuf,
    pub reason: String,
}

#[derive(Debug)]
pub struct UndoDirectoryFailure {
    pub directory_path: PathBuf,
    pub reason: String,
}

impl UndoResult {
    pub fn new() -> Self {
        Self {
            restored_files: Vec::new(),
            skipped_files: Vec::new(),
            removed_directories: Vec::new(),
            directory_failures: Vec::new(),
        }
    }

    pub fn has_warnings(&self) -> bool {
        !self.skipped_files.is_empty() || !self.directory_failures.is_empty()
    }
}

pub fn undo_operation(record: &OperationRecord) -> UndoResult {
    let mut result = UndoResult::new();

    for history_move in record.moves.iter().rev() {
        let from_path = history_move.destination_path.clone();
        let to_path = history_move.source_path.clone();

        if !from_path.exists() {
            result.skipped_files.push(UndoSkippedFile {
                from_path,
                to_path,
                reason: "organized file does not exist".to_string(),
            });
            continue;
        }

        if to_path.exists() {
            result.skipped_files.push(UndoSkippedFile {
                from_path,
                to_path,
                reason: "original path is occupied".to_string(),
            });
            continue;
        }

        match fs::rename(&from_path, &to_path) {
            Ok(()) => {
                result
                    .restored_files
                    .push(RestoredFile { from_path, to_path });
            }
            Err(error) => {
                result.skipped_files.push(UndoSkippedFile {
                    from_path,
                    to_path,
                    reason: format!("failed to restore file: {error}"),
                });
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::undo_operation;
    use crate::history::{HistoryMove, OperationRecord, OperationStatus};
    use std::fs;

    fn operation_record_with_moves(moves: Vec<HistoryMove>) -> OperationRecord {
        OperationRecord {
            version: 1,
            id: "operation-test".to_string(),
            target_directory: ".".into(),
            created_at: "operation-test".to_string(),
            completed_at: "operation-test".to_string(),
            created_directories: Vec::new(),
            moves,
            failures: Vec::new(),
            status: OperationStatus::Completed,
        }
    }

    #[test]
    fn restores_moved_file_to_original_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("report.pdf");
        let destination_dir = temp_dir.path().join("Documents");
        let destination = destination_dir.join("report.pdf");

        fs::create_dir(&destination_dir).unwrap();
        fs::write(&destination, "report").unwrap();

        let record = operation_record_with_moves(vec![HistoryMove {
            source_path: source.clone(),
            destination_path: destination.clone(),
            conflict_resolution: "None".to_string(),
            completed_at: "operation-test".to_string(),
        }]);

        let result = undo_operation(&record);

        assert_eq!(fs::read_to_string(&source).unwrap(), "report");
        assert!(!destination.exists());
        assert_eq!(result.restored_files.len(), 1);
        assert!(result.skipped_files.is_empty());
    }

    #[test]
    fn skips_missing_organized_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("report.pdf");
        let destination = temp_dir.path().join("Documents").join("report.pdf");

        let record = operation_record_with_moves(vec![HistoryMove {
            source_path: source,
            destination_path: destination,
            conflict_resolution: "None".to_string(),
            completed_at: "operation-test".to_string(),
        }]);

        let result = undo_operation(&record);

        assert!(result.restored_files.is_empty());
        assert_eq!(result.skipped_files.len(), 1);
        assert_eq!(
            result.skipped_files[0].reason,
            "organized file does not exist"
        );
    }

    #[test]
    fn skips_when_original_path_is_occupied() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source = temp_dir.path().join("report.pdf");
        let destination_dir = temp_dir.path().join("Documents");
        let destination = destination_dir.join("report.pdf");

        fs::create_dir(&destination_dir).unwrap();
        fs::write(&source, "occupied").unwrap();
        fs::write(&destination, "organized").unwrap();

        let record = operation_record_with_moves(vec![HistoryMove {
            source_path: source.clone(),
            destination_path: destination.clone(),
            conflict_resolution: "None".to_string(),
            completed_at: "operation-test".to_string(),
        }]);

        let result = undo_operation(&record);

        assert_eq!(fs::read_to_string(&source).unwrap(), "occupied");
        assert_eq!(fs::read_to_string(&destination).unwrap(), "organized");
        assert!(result.restored_files.is_empty());
        assert_eq!(result.skipped_files.len(), 1);
        assert_eq!(result.skipped_files[0].reason, "original path is occupied");
    }
}
